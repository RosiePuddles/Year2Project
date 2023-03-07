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

// Credit to
// https://codingvision.net/c-simple-tcp-server for TCP code
// and 

namespace HRConsole
{
    internal class Program
    {
        static DeviceInformation device;
        static int HR = 100;

        // Bluetooth HR services start with 180d
        static string HEART_RATE_SERVICE_ID = "180d";


        static void Tcp()
        {
            TcpListener server = new TcpListener(IPAddress.Parse("127.0.0.1"), 1234);
            // we set our IP address as server's address, and we also set the port: 1234

            server.Start(); // this will start the server
            while (true) //we wait for a connection
            {
                TcpClient client = server.AcceptTcpClient(); //if a connection exists, the server will accept it
                Console.WriteLine("Client Connected");
                NetworkStream ns = client.GetStream(); //networkstream is used to send/receive messages

                while (client.Connected) //while the client is connected, we look for incoming messages
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

        static void Main(string[] args)
        {
            Task TcpTask = new Task(Tcp);
            var HeartRateTask = HeartRateReadings();

            TcpTask.Start();

            HeartRateTask.Wait();
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
            Console.ReadKey();
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

        // Ran constantly when deviceWatcher.start() is ran in HeartRateReadings
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