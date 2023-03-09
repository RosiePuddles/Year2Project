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

    [SerializeField]
    static int baseLineHR;


    private bool changingColour;
    bool changingVelocity;
    private bool collectingHeartRates;
    bool collectingMeditation;

    [SerializeField]
    private float angularVelocity;
    public float radius = 10f;

    private Transform userTransform;
    private Vector3 centre;
    private float angle;


    private List<int> heartRateAverages = new();
    private List<int> meditationAverages = new();

    void Awake()
    {
        baseLineHR = 70; 
        cubeRenderer = gameObject.GetComponent<Renderer>();
        radius = CubeSpawner.cubeRadius;


        userTransform = Camera.main.transform;
        centre = userTransform.position;

        Vector3 diffVect = transform.position - new Vector3(0, 0, 1);

        if(diffVect.magnitude != 0)
        {
            angle = 2 * Mathf.Asin((diffVect.magnitude)/2);
        }
        else
        {
            angle = 0;
        }

        if(diffVect.x < 0)
        {
            angle = (2 * Mathf.PI) - angle;
        }

    }




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
        angle += angularVelocity * Time.deltaTime;
        float x = Mathf.Sin(angle) * radius;
        float y = 0f;
        float z = Mathf.Cos(angle) * radius;

        centre = userTransform.position;
        transform.position = centre + new Vector3(x, y, z);

        
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
        float time = 0f;

        meditationAverages.Clear();

        while (time < refreshDuration)
        {
            meditationAverages.Add(Myndplay.GetMeditationValue());
            time += Time.deltaTime;

            yield return null;
        }

        averageMeditation = Mathf.RoundToInt((float)meditationAverages.Average());
    }

    private float HeartRateSigmoid(int hr, int midpoint = 70, int scale = 5)
    {
        float x = (hr - midpoint) / scale;
        float sigmoid = 1 / (1 + Mathf.Exp(x));
        return sigmoid;
    }

    private float MeditationExponential(int med, int halfLife=50, float scalar = 1f)
    {
        float y = scalar * Mathf.Exp((-Mathf.Log(2) / halfLife) * med);
  
        return y;
    }
    public float MeditationLinear(int med, float floor=0.1f, float ceiling=1f)
    {
        float delta = ceiling- floor;
        float y = ceiling - (delta * ((float)med / 100));
        return y;

    }
    public static void ChangeBaseHR(int newBase)
    {
        if(newBase >30 && newBase < 150)
        {
            baseLineHR = newBase;
        }

    }
    IEnumerator ChangeColour()
    {
        changingColour = true;
        Debug.Log(Time.deltaTime + " HR Average:" + averageHeartRate);
        float heartRateScaled = HeartRateSigmoid(averageHeartRate, baseLineHR);
        Debug.Log("Baseline cube " + baseLineHR);



        Color oldColour = cubeRenderer.material.color;

        // at the moment this will just make it vary between white (low HR's) and black (high HR's)
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

        float meditationScaled = MeditationLinear(averageMeditation);
        Debug.Log(Time.deltaTime + " Meditation Average:" + meditationScaled);


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