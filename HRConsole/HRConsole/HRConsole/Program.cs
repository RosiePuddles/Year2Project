using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Windows.Devices.Enumeration;
using Windows.Devices.Bluetooth.Advertisement;
using Windows.Devices.Bluetooth;
using System.Threading;
using Windows.Devices.Bluetooth.GenericAttributeProfile;
using Windows.Media.Capture;
using Windows.Storage.Streams;

namespace HRConsole
{
    internal class Program
    {
        static DeviceInformation device = null;

        // Bluetooth HR services start with 180d
        public static string HEART_RATE_SERVICE_ID = "180d";

        static async Task Main(string[] args)
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


            while (true)
            {
                if (device == null)
                {
                    Thread.Sleep(200);
                }
                else
                {
                    Console.WriteLine("Press Any Key To Pair");
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

                                GattCharacteristicsResult characteristicsResults = await service.GetCharacteristicsAsync();

                                if (characteristicsResults.Status == GattCommunicationStatus.Success)
                                {
                                    var characteristics = characteristicsResults.Characteristics;
                                    foreach (var characteristic in characteristics)
                                    {
                                        Console.WriteLine("---------------");
                                        Console.WriteLine(characteristics);

                                        GattCharacteristicProperties properties = characteristic.CharacteristicProperties;


                                        if (properties.HasFlag(GattCharacteristicProperties.Notify))
                                        {
                                            Console.WriteLine("Notfiy property found");
                                            GattCommunicationStatus status = await characteristic.WriteClientCharacteristicConfigurationDescriptorAsync(
                        GattClientCharacteristicConfigurationDescriptorValue.Notify);
                                            if (status == GattCommunicationStatus.Success)
                                            {
                                                characteristic.ValueChanged += Characteristic_ValueChanged;
                                                // Server has been informed of clients interest.
                                            }

                                        }
                                    }

                                }

                            }

                        }



                    }
                    else
                    {
                        Console.WriteLine("unreachable");
                    }

                    Console.WriteLine("press any key to exit");
                    Console.ReadKey();
                    break;

                }

            }

        }

        private static void Characteristic_ValueChanged(GattCharacteristic sender, GattValueChangedEventArgs args)
        {
            var reader = DataReader.FromBuffer(args.CharacteristicValue);
            var flags = reader.ReadByte();
            var value = reader.ReadByte();
            Console.WriteLine($"{flags} - {value}");
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

        private static void DeviceWatcher_Added(DeviceWatcher sender, DeviceInformation args)
        {
            Console.WriteLine(args.Name);
            if (args.Name == "Polar H10 9E127E27")
            {
                device = args;
            }
            // throw new NotImplementedException();
        }
    }
}
