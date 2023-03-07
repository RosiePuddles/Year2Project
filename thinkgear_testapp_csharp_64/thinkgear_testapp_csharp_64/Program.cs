using libStreamSDK;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace thinkgear_testapp_csharp_64
{
    internal class Program
    {
        static int meditation = 0;
        static int count = 0;
        static int connectionID = 0;
        static void Tcp()
        {
            // Starts server on local host with port 8080
            TcpListener server = new TcpListener(IPAddress.Parse("127.0.0.1"), 8080);


            server.Start();  // this will start the server
            while (true)  
            {
                TcpClient client = server.AcceptTcpClient();  //if a connection exists, the server will accept it
                Console.WriteLine("Client Connected");

                NetworkStream ns = client.GetStream(); // networkstream is used to send/receive messages

                while (client.Connected)  //while the client is connected, we look for incoming messages
                {
                    try
                    {
                        Thread.Sleep(200); // send the message every 200 milliseconds
                        ns.Write(Encoding.ASCII.GetBytes(meditation.ToString()), 0, meditation.ToString().Length);
                    }
                    catch // exception occur when client disconnects
                    {
                        break;
                    }

                }
                Console.WriteLine("Client Disconnected");
            }
        }
        static int meditationCalc()
        {
            int errCode = 0;
            if (count == 0)
            {
                NativeThinkgear thinkgear = new NativeThinkgear();

                /* Print driver version number */
                Console.WriteLine("Version: " + NativeThinkgear.TG_GetVersion());

                /* Get a connection ID handle to ThinkGear */
                connectionID = NativeThinkgear.TG_GetNewConnectionId();
                Console.WriteLine("Connection ID: " + connectionID);

                if (connectionID < 0)
                {
                    Console.WriteLine("ERROR: TG_GetNewConnectionId() returned: " + connectionID);
                    return 0;
                }

                
                /* Set/open stream (raw bytes) log file for connection */

                // Change this to the outgoing port of the myndplay headband
                // This can be found by going to: Bluetooth->More Bluetooth Settings
                // Then a window will open. Check for the COM port with "Mynband 'Bluetooth Serial Port'
                // and direction outgoing.

                string comPortName = "COM5";

                errCode = NativeThinkgear.TG_Connect(connectionID,
                              comPortName,
                              NativeThinkgear.Baudrate.TG_BAUD_57600,
                              NativeThinkgear.SerialDataFormat.TG_STREAM_PACKETS);
                if (errCode < 0)
                {
                    Console.WriteLine("ERROR: TG_Connect() returned: " + errCode);
                    return meditation;
                }
                count++;
            }
            /* Read 10 ThinkGear Packets from the connection, 1 Packet at a time */

            errCode = NativeThinkgear.TG_EnableAutoRead(connectionID, 1);
            if (errCode == 0)
            {
                // it use as time
                int readPackets = 0; 
                while (readPackets < 2000)
                {
                    if (NativeThinkgear.TG_GetValueStatus(connectionID, NativeThinkgear.DataType.TG_DATA_RAW) != 0)
                    {
                        meditation = (int)NativeThinkgear.TG_GetValue(connectionID, NativeThinkgear.DataType.TG_DATA_MEDITATION);
                        return meditation;
                    }
                }
                
                /* If raw value has been updated ... */
            }
            else
            {
                return meditation;
            }

            NativeThinkgear.TG_Disconnect(connectionID); // disconnect 
            NativeThinkgear.TG_FreeConnection(connectionID);
            return meditation;

        }
        private static void MyndplayReadings()
        {
            while (true)  //while the client is connected, we look for incoming messages
            {
                try
                {
                    Thread.Sleep(200); //Take a meditation reading every 200ms
                    meditation = meditationCalc();
                    Console.WriteLine($"Meditation Value: {meditation}");
                }
                catch //exception occur when client disconnects
                {
                    break;
                }
            }
        }
        static void Main(string[] args)
        {
            Task TCP = new Task(Tcp);
            Task Myndplay = new Task(MyndplayReadings);

            TCP.Start();
            Myndplay.Start();

            Console.ReadKey();

        }
    }
}

