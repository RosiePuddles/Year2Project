using System;
using System.Linq;
using System.Text;
using UnityEngine;
using UnityEngine.Networking;

namespace Requests
{
    /// <summary>
    /// Request error class. Extends the <see cref="Exception"/> class and raised on a error 
    /// </summary>
    public class Error : Exception
    {
        /// <summary>
        /// <see cref="ErrorType"/> error type
        /// </summary>
        public ErrorType kind { get; set; }

        public Error(ErrorType Kind) => kind = Kind;
    }

    /// <summary>
    /// Request error type
    /// </summary>
    public enum ErrorType
    {
        /// <summary>
        /// 500 response
        /// </summary>
        ServerError,

        /// <summary>
        /// 404 response
        /// </summary>
        NotFound,

        /// <summary>
        /// 400 response. Given data was not able to be serialised
        /// </summary>
        BadData,

        /// <summary>
        /// User conflict
        /// </summary>
        Conflict,

        /// <summary>
        /// Connection error
        /// </summary>
        Connection,

        /// <summary>
        /// Protocol error
        /// </summary>
        Protocol,

        /// <summary>
        /// Default <code>ErrorKind</code> variant
        /// </summary>
        Unhandled
    }

    /// <summary>
    /// Request class. C# doesn't allow exporting single function so this is where all the requests are made from
    /// </summary>
    /// <example><code>
    /// // login and get user key
    /// LoginKey key = Request.Login("username");
    /// // submit session with api key
    /// LoginKey key = Request.SubmitSession(key, DateTimeOffset.Now, [], []);
    /// </code></example>
    public static class Request
    {
        /// <summary>
        /// Make a login request to the server. Returns a user login key required for further API use by the user
        /// </summary>
        /// <param name="uname">Username to login with</param>
        /// <returns>User login API key</returns>
        /// <exception cref="Error">Raised if an error occurs with the request. See <see cref="ErrorType"/></exception>
        public static LoginKey Login(string uname)
        {
            using (UnityWebRequest www = UnityWebRequest.Post("http://127.0.0.1:8080/api/login", ""))
            {
                byte[] bytes = Encoding.UTF8.GetBytes($"{{\"uname\":\"{uname}\"}}");
                www.uploadHandler = new UploadHandlerRaw(bytes);
                www.uploadHandler.contentType = "text/json";
                www.SendWebRequest();

                if (www.result != UnityWebRequest.Result.Success)
                    throw www.result switch
                    {
                        UnityWebRequest.Result.ConnectionError => new Error(ErrorType.Connection),
                        UnityWebRequest.Result.ProtocolError => new Error(ErrorType.Protocol),
                        _ => new Error(ErrorType.Unhandled)
                    };
                if (www.responseCode == 200)
                    return JsonUtility.FromJson<LoginKeyIntermediate>(www.downloadHandler.text).ToReal();
                throw www.responseCode switch
                {
                    400 => new Error(ErrorType.BadData),
                    404 => new Error(ErrorType.NotFound),
                    409 => new Error(ErrorType.Conflict),
                    500 => new Error(ErrorType.ServerError),
                    _ => new Error(ErrorType.Unhandled)
                };
            }
        }

        /// <summary>
        /// Make a new user request to the server. Returns a user login key required for further API use by the user
        /// </summary>
        /// <param name="uname"></param>
        /// <returns>User login API key</returns>
        /// <exception cref="Error">Raised if an error occurs with the request. See <see cref="ErrorType"/></exception>
        public static LoginKey NewUser(string uname)
        {
            using (UnityWebRequest www = UnityWebRequest.Post("http://127.0.0.1:8080/api/new", ""))
            {
                byte[] bytes = Encoding.UTF8.GetBytes($"{{\"uname\":\"{uname}\"}}");
                www.uploadHandler = new UploadHandlerRaw(bytes);
                www.uploadHandler.contentType = "text/json";
                www.SendWebRequest();

                if (www.result != UnityWebRequest.Result.Success)
                    throw www.result switch
                    {
                        UnityWebRequest.Result.ConnectionError => new Error(ErrorType.Connection),
                        UnityWebRequest.Result.ProtocolError => new Error(ErrorType.Protocol),
                        _ => new Error(ErrorType.Unhandled)
                    };
                if (www.responseCode == 200)
                    return JsonUtility.FromJson<LoginKeyIntermediate>(www.downloadHandler.text).ToReal();
                throw www.responseCode switch
                {
                    400 => new Error(ErrorType.BadData),
                    404 => new Error(ErrorType.NotFound),
                    409 => new Error(ErrorType.Conflict),
                    500 => new Error(ErrorType.ServerError),
                    _ => new Error(ErrorType.Unhandled)
                };
            }
        }

        /// <summary>
        /// Make a new user request to the server. Returns a user login key required for further API use by the user
        /// </summary>
        /// <param name="key">User key to submit the API call with</param>
        /// <param name="startTime">Time the session started</param>
        /// <param name="heartRate">Heart rate data</param>
        /// <param name="gaze">Gaze data <b>This may change type!</b></param>
        /// <returns>Nothing is returned on a success</returns>
        /// <exception cref="Error">Raised if an error occurs with the request. See <see cref="ErrorType"/></exception>
        public static void SubmitSession(LoginKey key, DateTimeOffset startTime, int[] heartRate, (int, int)[] gaze)
        {
            using (UnityWebRequest www = UnityWebRequest.Post("http://127.0.0.1:8080/api/submit", ""))
            {
                string startTimeString = startTime.ToString("o");
                string heartRateString = string.Join(",", heartRate.Select(hr => hr.ToString()));
                string gazeString =
                    string.Join(",", gaze.Select(gazePoint => $"[{gazePoint.Item1},{gazePoint.Item2}]"));
                string body =
                    $"{{\"key\":\"{key.Key}\",\"time\":\"{startTimeString}\",\"hr\":[{heartRateString}],\"gaze\":[{gazeString}]}}";

                byte[] bytes = Encoding.UTF8.GetBytes(body);
                www.uploadHandler = new UploadHandlerRaw(bytes);
                www.uploadHandler.contentType = "text/json";
                www.SendWebRequest();

                if (www.result != UnityWebRequest.Result.Success)
                    throw www.result switch
                    {
                        UnityWebRequest.Result.ConnectionError => new Error(ErrorType.Connection),
                        UnityWebRequest.Result.ProtocolError => new Error(ErrorType.Protocol),
                        _ => new Error(ErrorType.Unhandled)
                    };
                if (www.responseCode == 200)
                    return;
                throw www.responseCode switch
                {
                    400 => new Error(ErrorType.BadData),
                    404 => new Error(ErrorType.NotFound),
                    409 => new Error(ErrorType.Conflict),
                    500 => new Error(ErrorType.ServerError),
                    _ => new Error(ErrorType.Unhandled)
                };
            }
        }
    }

    /// <summary>
    /// <see cref="LoginKey"/> intermediate type to allow for JSON (de)serialisation.
    /// </summary>
    [Serializable]
    class LoginKeyIntermediate
    {
        public string key;
        public string time;

        /// <summary>
        /// Returns a new <see cref="LoginKey"/> instance from the contained data
        /// </summary>
        /// <returns><see cref="LoginKey"/> equivalent instance</returns>
        public LoginKey ToReal()
        {
            return new LoginKey(key, DateTimeOffset.Parse(time));
        }
    }

    /// <summary>
    /// User login key
    /// This is required for any user specific API requests
    /// </summary>
    public class LoginKey
    {
        /// <summary>
        /// User API key
        /// </summary>
        public string Key { get; }

        /// <summary>
        /// Expiration time for the key
        /// </summary>
        public DateTimeOffset Time { get; }

        public LoginKey(string key, DateTimeOffset time)
        {
            Key = key;
            Time = time;
        }

        LoginKeyIntermediate ToIntermediate()
        {
            LoginKeyIntermediate final = new LoginKeyIntermediate();
            final.key = Key;
            final.time = Time.ToString("o");
            return final;
        }
    }
}
