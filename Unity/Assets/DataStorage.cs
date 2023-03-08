using Mono.Cecil;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using UnityEngine.Audio;
using UnityEngine.SceneManagement;
public class DataStorage : MonoBehaviour
{
    [SerializeField]
    GameObject heartRateController;
    [SerializeField]
    GameObject EEGController;
    [SerializeField]
    AudioSource meditiationAudio;
    List<(string, int)> heartRateReadings = new List<(string, int)>();
    List<(string, int)> EEGReadings = new List<(string, int)>();

    // Start is called before the first frame update
    void Update()
    {
        if (!meditiationAudio.isPlaying)
        {
            heartRateReadings = HeartRateSensor.hrReadings.ToList();
            EEGReadings = Myndplay.EEGreadings.ToList();

            Destroy(heartRateController);
            Destroy(EEGController);
            foreach(var reading in heartRateReadings)
            {
                Debug.Log("hr " +reading);
            }

            foreach (var reading in EEGReadings)
            {
                Debug.Log("EEG " + reading);
            }
            SceneManager.LoadScene("Menu");
        }
    }

    
}
