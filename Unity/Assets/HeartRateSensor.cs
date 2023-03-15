﻿using System;
using System.Net.Sockets;
using System.Text;
using UnityEngine;
using System.Threading.Tasks;
using System.Threading;
using System.Collections.Generic;



class HeartRateSensor : MonoBehaviour
{

    // credit to https://gist.github.com/danielbierwirth/0636650b005834204cb19ef5ae6ccedb
    // for code to create TCP client connection


    private TcpClient socketConnection;
    private Task listenTask;
    private CancellationTokenSource cancellationTokenSource;

    // contains list of tuples of the time and heart rate
    public static List<(string, int)> hrReadings { get; private set; }

    private static int HeartRate;

    // Use this for initialization 	
    void Start()
    {
        ConnectToTcpServer();
    }

    private void ConnectToTcpServer()
    {
        try
        {
            hrReadings = new List<(string, int)>();
            cancellationTokenSource = new CancellationTokenSource();
            var CancellationToken = cancellationTokenSource.Token;

            // Run a task to listen out for data
            listenTask = Task.Run(() => ListenForData(CancellationToken));
        }
        catch (Exception e)
        {
            Debug.Log("On client connect exception " + e);
        }
    }

    private void OnDestroy()
    {
        // This script is destroyed when returning from the meditation back to the main menu
        // so we need to cancel the task and close the socket connection
        cancellationTokenSource.Cancel();
        if (socketConnection != null)
        {
            socketConnection.Close();
        }
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
                                try
                                {
                                    // message is appended with B, if it is the intial base line heart rate
                                    if (serverMessage[0] == 'B') {
                                        CubeController.ChangeBaseHR(int.Parse(serverMessage.Substring(1)));
                                        Debug.Log("Base line " + int.Parse(serverMessage.Substring(1)));
                                    }
                                    else
                                    {
                                        throw new Exception("The first character is not B");
                                    }
                                }
                                catch(Exception ex) 
                                {
                                    Debug.Log(ex.Message);
                                }
                             
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