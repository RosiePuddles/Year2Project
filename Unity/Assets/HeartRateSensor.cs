using System;
using System.Collections;
using System.Collections.Generic;
using System.Net.Sockets;
using System.Net;
using System.Text;
using UnityEngine;
using System.Threading.Tasks;
using UnityEditor.Networking.PlayerConnection;
using System.Threading;
using Unity.VisualScripting;

// Credit to
// https://gist.github.com/danielbierwirth/0636650b005834204cb19ef5ae6ccedb
class HeartRateSensor : MonoBehaviour
{
    private TcpClient socketConnection;
    private Task listenTask;
    private CancellationTokenSource cancellationTokenSource;

    private static int HeartRate;

    // Use this for initialization 	
    void Start()
    {
        ConnectToTcpServer();
    }
    // Update is called once per frame
    void Update()
    {
        if (Input.GetKeyDown(KeyCode.Space))
        {
            SendMessage();
        }
    }
    /// <summary> 	
    /// Setup socket connection. 	
    /// </summary> 	
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
    /// <summary> 	
    /// Runs in background clientReceiveThread; Listens for incomming data. 	
    /// </summary>

    private void OnDestroy()
    {

        cancellationTokenSource.Cancel();
        listenTask.Wait();
        socketConnection.Close();
        socketConnection.Dispose();



    }
    private void ListenForData(CancellationToken cancellationToken)
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
                        //Debug.Log("server message received as: " + serverMessage);

                        if (int.TryParse(serverMessage, out HeartRate))
                        {
                            //Debug.Log(HeartRate);
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
        catch (SocketException socketException)
        {
            Debug.Log("Socket exception: " + socketException);
        }
    }
    /// <summary> 	
    /// Send message to server using socket connection. 	
    /// </summary> 	
    private void SendMessage()
    {
        if (socketConnection == null)
        {
            return;
        }
        try
        {
            // Get a stream object for writing. 			
            NetworkStream stream = socketConnection.GetStream();
            if (stream.CanWrite)
            {
                string clientMessage = "This is a message from one of your clients.";
                // Convert string message to byte array.                 
                byte[] clientMessageAsByteArray = Encoding.ASCII.GetBytes(clientMessage);
                // Write byte array to socketConnection stream.                 
                stream.Write(clientMessageAsByteArray, 0, clientMessageAsByteArray.Length);
                Debug.Log("Client sent his message - should be received by server");
            }
        }
        catch (SocketException socketException)
        {
            Debug.Log("Socket exception: " + socketException);
        }
    }


    public static int GetHeartRate()
    {
        return HeartRate;
    }
}


