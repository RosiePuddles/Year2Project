using System;
using System.Text;
using System.Threading.Tasks;
using Windows.Devices.Enumeration;
using Windows.Devices.Bluetooth;
using System.Threading;
using Windows.Devices.Bluetooth.GenericAttributeProfile;
using Windows.Storage.Streams;
using System.Net.Sockets;
using System.Net;
using System.Collections.Generic;
using System.Linq;

// Credit to
// https://codingvision.net/c-simple-t
// cp-server for TCP code


namespace HRConsole
{
    internal class Program
    {
        static DeviceInformation device;
        static int HR = 100;
        static int baseLineHR = -1;
        // Bluetooth HR services start with 180d
        static string HEART_RATE_SERVICE_ID = "180d";
        static List<int> baselineValues  =new List<int>();
        static bool takeBaseline = false;


        static void Main(string[] args)
        {
            var HeartRateTask = HeartRateReadings();
            Task TcpTask = new Task(Tcp);
            TcpTask.Start();

            HeartRateTask.Wait();
        }
        static void Tcp()
        {
            TcpListener server = new TcpListener(IPAddress.Parse("127.0.0.1"), 1234);
            // we set our IP address as server's address, and we also set the port: 1234

            server.Start(); // this will start the server


            while (true) 
            {
                TcpClient client = server.AcceptTcpClient(); //if a connection exists, the server will accept it
                Console.WriteLine("Client Connected");
                NetworkStream ns = client.GetStream(); //networkstream is used to send/receive messages


                // here we check if the base line heart rate has been set
                // if it has, then this is the first thing we send to Unity
                if (baseLineHR != -1)
                {
                    try
                    {
                        // We put a B in front of the heart rate to indicate that this is the base line heart rate
                        string message = "B" + baseLineHR.ToString();
                        ns.Write(Encoding.ASCII.GetBytes(message), 0, message.Length);
                    }
                    catch
                    {
                        break;
                    }
                }


                while (client.Connected) // while the client is connected, we regularly send 
                {    
                    Thread.Sleep(200); // send the message every 200 milliseconds
                    try
                    {
                        ns.Write(Encoding.ASCII.GetBytes(HR.ToString()), 0, HR.ToString().Length);
                    }
                    catch //exception occur when client disconnects
                    {
                        break;
                    }
                }

                Console.WriteLine("Client Disconnected");
            }
        }



        static async Task HeartRateReadings()
        {

            // First we want to detect near by devices
            // Query for extra properties you want returned
            string[] requestedProperties = { "System.Devices.Aep.DeviceAddress", "System.Devices.Aep.IsConnected" };
            DeviceWatcher deviceWatcher =
                DeviceInformation.CreateWatcher(
                    BluetoothLEDevice.GetDeviceSelectorFromPairingState(false),
                    requestedProperties,
                    DeviceInformationKind.AssociationEndpoint);

            // Register event handlers before starting the watcher.
            // Added, Updated and Removed are required to get all nearby devices
            deviceWatcher.Added += DeviceWatcher_Added;
            deviceWatcher.Updated += DeviceWatcher_Updated;
            deviceWatcher.Removed += DeviceWatcher_Removed;

            // EnumerationCompleted and Stopped are optional to implement.
            deviceWatcher.EnumerationCompleted += DeviceWatcher_EnumerationCompleted;
            deviceWatcher.Stopped += DeviceWatcher_Stopped;

            // Start the watcher.
            deviceWatcher.Start();

            while(device == null)
            {
                Thread.Sleep(200);
            }
            deviceWatcher.Stop();
            Console.WriteLine("\nPress Any Key To Pair");
            Console.ReadKey(true);
            BluetoothLEDevice bluetoothLeDevice = await BluetoothLEDevice.FromIdAsync(device.Id);
            Console.WriteLine("Attempting to Pair");

            GattDeviceServicesResult result = await bluetoothLeDevice.GetGattServicesAsync();

            // Ensures bluetooth can pair
            if (result.Status == GattCommunicationStatus.Success)
            {
                Console.WriteLine("Pairing Succesfull");
                var services = result.Services;
                foreach (var service in services)
                {
                    // Get the correct service (i.e the one that has the HR)

                    if (service.Uuid.ToString().Substring(4, 4) == HEART_RATE_SERVICE_ID)
                    {
                        Console.WriteLine("Found Heart Rate Service");
                        GattCharacteristicsResult characteristicsResults =
                            await service.GetCharacteristicsAsync();

                        if (characteristicsResults.Status == GattCommunicationStatus.Success)
                        {
                            Thread.Sleep(200);
                            var characteristics = characteristicsResults.Characteristics;
                            foreach (var characteristic in characteristics)
                            {
                                GattCharacteristicProperties properties =
                                    characteristic.CharacteristicProperties;


                                if (properties.HasFlag(GattCharacteristicProperties.Notify))
                                {
                                    Console.WriteLine("Notfiy property found");
                                    GattCommunicationStatus status =
                                        await characteristic
                                            .WriteClientCharacteristicConfigurationDescriptorAsync(
                                                GattClientCharacteristicConfigurationDescriptorValue.Notify);
                                    if (status == GattCommunicationStatus.Success)
                                    {

                                        characteristic.ValueChanged += Characteristic_ValueChanged;
                                        Console.WriteLine("Do you want to take a baseline heart rate? Reply either <y> or <n>");
                                        ConsoleKeyInfo keyInfo = Console.ReadKey(true);
                                        if (keyInfo.KeyChar == 'y')
                                        {
                                            takeBaseline = true;
                                            Console.WriteLine("Please wait 90 seconds");

                                            Thread.Sleep(900);
                                            Console.WriteLine("Baseline heart rate taken");
                                            baseLineHR = (int)Math.Round(baselineValues.Average());

                                            takeBaseline = false;
                                            baselineValues.Clear();

                                        }
                                        else
                                        {
                                            Console.WriteLine("Baseline heart rate not taken");
                                        }
                                        // Server has been informed of clients interest.
                                    }
                                }
                            }
                        }
                        else
                        {
                            Console.WriteLine("Characteristics not detected. Please try again");
                        }
                    }
                }



            }
            else
            {
                Console.WriteLine("device unreachable");
            }


            Console.WriteLine("\npress any key to exit");
            Console.ReadKey();

        }

        // This method is ran constantly after it is first called in HeartRateReadings, and continues on after
        // that method closes
        private static void Characteristic_ValueChanged(GattCharacteristic sender, GattValueChangedEventArgs args)
        {
            var reader = DataReader.FromBuffer(args.CharacteristicValue);
            var flags = reader.ReadByte();
            var value = reader.ReadByte();
            HR = value;
            if(takeBaseline) // takes baseline values untill explicitly told not to
            {
                baselineValues.Add(value);
            }

            Console.WriteLine($"Heart Rate: {value}");


        }

        private static void DeviceWatcher_Stopped(DeviceWatcher sender, object args)
        {
            //throw new NotImplementedException();
        }

        private static void DeviceWatcher_EnumerationCompleted(DeviceWatcher sender, object args)
        {
            //throw new NotImplementedException();
        }

        private static void DeviceWatcher_Removed(DeviceWatcher sender, DeviceInformationUpdate args)
        {
            //throw new NotImplementedException();
        }

        private static void DeviceWatcher_Updated(DeviceWatcher sender, DeviceInformationUpdate args)
        {
            //throw new NotImplementedException();
        }

        // Ran a few times when deviceWatcher.start() is ran in HeartRateReadings
        // deviceWatcher.stop() is called after device is no longer null (which occurs, when the Polar H10 is found)
        private static void DeviceWatcher_Added(DeviceWatcher sender, DeviceInformation args)
        {
            string deviceName = args.Name;
            if (deviceName != "")
            {
                Console.WriteLine(deviceName);
                if (deviceName == "Polar H10 9E127E27")
                {
                    device = args;
                }
            }
        }
    }
}