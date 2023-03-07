using System;
using System.Net.Sockets;
using System.Text;
using UnityEngine;
using System.Threading.Tasks;
using System.Threading;

public class Myndplay : MonoBehaviour
{
    private TcpClient socketConnection;
    private Task listenTask;
    private CancellationTokenSource cancellationTokenSource;

    private static int meditationValue;

    // Start is called before the first frame update
    void Start()
    {
        ConnectToTcpServer();
    }

    private void ConnectToTcpServer()
    {
        try
        {
            cancellationTokenSource = new CancellationTokenSource();
            var CancellationToken = cancellationTokenSource.Token;
            listenTask = Task.Run(() => ListenForData(CancellationToken));
        
        }
        catch (Exception e)
        {
            Debug.Log("On client connect exception " + e);
        }
    }

    private void OnDestroy()
    {
        cancellationTokenSource.Cancel();
    }

    private void ListenForData(CancellationToken cancellationToken)
    {
        while (true) // constantly try and connect to port 8080
        {
            try
            {
                socketConnection = new TcpClient("127.0.0.1", 8080);
                Byte[] bytes = new Byte[4];
                while (true)
                {
                    // Get a stream object for reading 				
                    using (NetworkStream stream = socketConnection.GetStream())
                    {
                        int length;
                        // Read incomming stream into byte arrary. 					
                        while ((length = stream.Read(bytes, 0, bytes.Length)) != 0)
                        {
                            var incommingData = new byte[length];
                            Array.Copy(bytes, 0, incommingData, 0, length);
                            // Convert byte array to string message. 						
                            string serverMessage = Encoding.ASCII.GetString(incommingData);

                            if (int.TryParse(serverMessage, out meditationValue))
                            {
                                // Debug.Log("meditation:"meditationValue);
                            }
                            else
                            {
                                Debug.Log("Could not parse");
                            }


                            if (cancellationToken.IsCancellationRequested)
                            {
                                return;
                            }
                        }
                    }
                }
            }
            catch
            {
                Debug.Log("Socket exception: Cannot find server for Meditation");
            }
        }

    }
    public static int GetMeditationValue()
    {
        return meditationValue;
    }
}