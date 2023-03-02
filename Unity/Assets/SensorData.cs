using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SensorData:MonoBehaviour
{
    // assumed rate at which data will come in at
    [SerializeField]
    private float dataRate = 0.2f;

    private int meditationValue;
    private int stepSize = 30;
    private int minHeartRate = 50;
    private int maxHeartRate = 150;
    private int heartRate = 100;

    private void Update()
    {
        StartCoroutine(GenerateHeartRate());
    }

    IEnumerator GenerateHeartRate()
    {
        // fake HR data obtained via 'random walk'
        // stochastic process


        // determine whether to increase or reduce HR
        int plusOrMinus = Random.Range(0, 1+1);


        if(plusOrMinus == 0)
        {
            heartRate += Random.Range(0,stepSize+1);
        }
        else
        {
            heartRate -= Random.Range(0,stepSize+1);
        }


        // make sure within bounds
        if(heartRate > maxHeartRate)
        {
            heartRate = maxHeartRate;
        }
        else if(heartRate < minHeartRate)
        {
            heartRate = minHeartRate;
        }

        // presumably data will come in at frequent intervals
        // i.e 1 second

        yield return new WaitForSeconds(1f);
    }
    public int GetHeartRate()
    {
        return heartRate;
    }

    // to be implemented later
    public int GetConcentrationVal()
    {
        return meditationValue;
    }
    public int GetMeditationVal()
    {
        return meditationValue;
    }

}
