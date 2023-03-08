using System;
using System.Net.Sockets;
using System.Text;
using UnityEngine;
using System.Threading.Tasks;
using System.Threading;
using System.Collections.Generic;

// Credit to
// https://gist.github.com/danielbierwirth/0636650b005834204cb19ef5ae6ccedb
class HeartRateSensor : MonoBehaviour
{
    private TcpClient socketConnection;
    private Task listenTask;
    private CancellationTokenSource cancellationTokenSource;

    public static List<(string, int)> hrReadings { get; private set; }

    private static int HeartRate;

    // Use this for initialization 	
    void Start()
    {
        ConnectToTcpServer();
    }

    private void ConnectToTcpServer()
    {
        hrReadings = new List<(string, int)>();
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
        socketConnection.Close();
    }

    private void ListenForData(CancellationToken cancellationToken)
    {
        while(true) // constantly try and connect to port 1234
        {
            try
            {
                socketConnection = new TcpClient("127.0.0.1", 1234);
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

                            if (cancellationToken.IsCancellationRequested)
                            {
                                socketConnection.Close();
                                return;
                            }


                            if (int.TryParse(serverMessage, out HeartRate))
                            {
                                hrReadings.Add((DateTime.Now.ToString(), HeartRate));
                            }
                            else
                            {
                                Debug.Log("Could not parse");
                            }



                        }
                    }
                }
            }
            catch
            {
                Debug.Log("Socket exception: Cannot find server for Heart Rate");
            }


        }
 
    }

    public static int GetHeartRate()
    {
        return HeartRate;
    }
}