using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Linq;
using Unity.VisualScripting;
using UnityEngine.PlayerLoop;

public class CubeController : MonoBehaviour
{
    private Renderer cubeRenderer;
    [SerializeField] private float refreshDuration = 3f;
    private int averageHeartRate;
    private int averageMeditation;


    private bool changingColour;
    bool changingVelocity;
    private bool collectingHeartRates;
    bool collectingMeditation;

    private List<int> heartRateAverages = new();
    private List<int> meditationAverages = new();

    void Start()
    {
        cubeRenderer = gameObject.GetComponent<Renderer>();

        userTransform = Camera.main.transform;
        center = userTransform.position;
    }


    public float angularVelocity;
    public float radius = 10f;

    private Transform userTransform;
    private Vector3 center;
    private float angle = 0f;

    void Update()

    {


        // only start a coroutine if it has finished from last time
        if (!collectingHeartRates)
        {
            StartCoroutine(AverageHeartRate());
        }
        if (!collectingMeditation)
        {
            StartCoroutine(AverageMeditation());
        }
        if (!changingColour)
        {
            StartCoroutine(ChangeColour());
        }
        if (!changingVelocity)
        {
            StartCoroutine(ChangeVelocity());
        }
    }

     void FixedUpdate()
    {
        Debug.Log(Time.deltaTime + " Angular Velocity " + angularVelocity);
        angle += angularVelocity * Time.deltaTime;
        float x = Mathf.Sin(angle) * radius;
        float y = 0f;
        float z = Mathf.Cos(angle) * radius;

        center = userTransform.position;
        transform.position = center + new Vector3(x, y, z);
    }


    IEnumerator AverageHeartRate()
    {
        collectingHeartRates = true;
        float time = 0f;

        heartRateAverages.Clear();

        while (time < refreshDuration)
        {
            heartRateAverages.Add(HeartRateSensor.GetHeartRate());
            time += Time.deltaTime;

            yield return null;
        }

        averageHeartRate = Mathf.RoundToInt((float)heartRateAverages.Average());
        collectingHeartRates = false;
    }

    IEnumerator AverageMeditation()
    {
        collectingMeditation = true;
        float time = 0f;

        meditationAverages.Clear();

        while (time < refreshDuration)
        {
            meditationAverages.Add(Myndplay.GetMeditationValue());
            time += Time.deltaTime;

            yield return null;
        }

        averageMeditation = Mathf.RoundToInt((float)meditationAverages.Average());
        Debug.Log(Time.deltaTime + " Meditation Average:" + averageMeditation);
        collectingMeditation = false;
    }

    private float HeartRateSigmoid(int hr, int midpoint = 70, int scale = 5)
    {
        float x = (hr - midpoint) / scale;
        float sigmoid = 1 / (1 + Mathf.Exp(-x));
        return sigmoid;
    }

    private float MeditationExponential(int med, int halfLife=50, float scalar = 2)
    {
        float y = scalar * Mathf.Exp((-Mathf.Log(2) / halfLife) * med);
  
        return y;
    }

    IEnumerator ChangeColour()
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


    IEnumerator ChangeVelocity()
    {
        changingVelocity = true;

        float meditationScaled = MeditationExponential(averageMeditation);


        float oldAngularVelocity = angularVelocity;
        // at the moment this will just make it vary between black (low HR's) and white (high HR's)
        float newAngularVelocity = meditationScaled;

        float time = 0f;

        while (time < refreshDuration)
        {
            // linearly interpolate between old and new colour
            angularVelocity = Mathf.Lerp(oldAngularVelocity,newAngularVelocity, time / refreshDuration);
            time += Time.deltaTime;

            yield return null;
        }

        angularVelocity = newAngularVelocity;
        changingVelocity = false;
    }
}