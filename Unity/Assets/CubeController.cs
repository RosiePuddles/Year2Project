using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Linq;
using UnityEditor.PackageManager;

public class CubeController : MonoBehaviour
{
    private Renderer cubeRenderer;
    private SensorData sensorScript;
    [SerializeField]
    private float refreshDuration = 3f;
    private int averageHeartRate = 0;

    private bool changingColour = false;
    private bool collectingHeartRates = false;

    private List<int> heartRateAverages = new List<int>();

    void Start()
    {
        cubeRenderer = gameObject.GetComponent<Renderer>();
    }
    
    void Update()
    {
        // only start a coroutine if it has finished from last time
        if (!collectingHeartRates)
        {
            StartCoroutine(AverageHeartRate());
        }
        if (!changingColour)
        {
            StartCoroutine(UpdateCube());
        }

    }

    IEnumerator AverageHeartRate()
    {
        collectingHeartRates = true;
        float time = 0f;

        heartRateAverages.Clear();

        while(time < refreshDuration)
        {
            heartRateAverages.Add(HeartRateSensor.GetHeartRate());
            time += Time.deltaTime;

            yield return null;
        }

        averageHeartRate = Mathf.RoundToInt((float)heartRateAverages.Average());
        collectingHeartRates = false;
    }
    
    private float HeartRateSigmoid(int hr, int midpoint=70, int scale=5)
    { 
        float x = (hr-midpoint) / scale;
        float sigmoid = 1 / (1 + Mathf.Exp(-x));
        return sigmoid;
    }

    IEnumerator UpdateCube()
    {
        changingColour = true;
        Debug.Log(Time.deltaTime + " HR Average:" + averageHeartRate);
        //float heartRateNorm = (float)averageHeartRate / 100;
        float heartRateScaled = HeartRateSigmoid(averageHeartRate);
        //heartRateNorm *= 2.5f;
        // Debug.Log("HR NORm " + heartRateNorm);

        Color oldColour = cubeRenderer.material.color;

        // at the moment this will just make it vary between black (low HR's) and white (high HR's)
        Color newColor = new Color(heartRateScaled, heartRateScaled, heartRateScaled);
        
        float time = 0f;

        while (time < refreshDuration)
        {
            // linearly interpolate between old and new colour

            cubeRenderer.material.color = Color.Lerp(oldColour, newColor, time / refreshDuration);
            time += Time.deltaTime;

            yield return null;
        }
        cubeRenderer.material.color = newColor;
        changingColour = false;

        // help from https://gamedevbeginner.com/the-right-way-to-lerp-in-unity-with-examples/#lerp_material_colour
    }
}
