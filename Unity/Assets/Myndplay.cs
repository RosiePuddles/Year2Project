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

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKeyDown(KeyCode.Space))
        {
            SendMessage();
        }
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

                        if (int.TryParse(serverMessage, out meditationValue))
                        {
                            Debug.Log(meditationValue);
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


    public static int GetmeditationValue()
    {
        return meditationValue;
    }
}

