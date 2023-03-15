using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Linq;
using Unity.VisualScripting;
using UnityEngine.PlayerLoop;

public class CubeController : MonoBehaviour
{
    // This script is attached to each cube in the scene
    // and controls its colour and movement
    // as dictated by the heart rate and meditation value respectively
    
    private Renderer cubeRenderer;
    [SerializeField] private float refreshDuration = 3f;
    private int averageHeartRate;
    private int averageMeditation;

    [SerializeField]
    static int baseLineHR;


    private bool changingColour;
    private bool changingVelocity;
    private bool collectingHeartRates;
    private bool collectingMeditation;

    [SerializeField]
    private float angularVelocity;
    public float radius = 10f;

    private Transform userTransform;
    private Vector3 centre;
    private float angle;


    private List<int> heartRateAverages = new();
    private List<int> meditationAverages = new();

    // awake called as first thing
    void Awake()
    {
        // initialised to this (in case no baseline is set)
        baseLineHR = 70; 

        cubeRenderer = gameObject.GetComponent<Renderer>();
        radius = CubeSpawner.cubeRadius;


        userTransform = Camera.main.transform;
        centre = userTransform.position;

        // each cube, when instatiated is given a position on a unit circle
        // we can calulate the angle of this position, with resepect to the forward facing vector (0,0,1)

        Vector3 diffVect = transform.position - new Vector3(0, 0, 1);

        // if the difference vector is zero, then the angle is zero
        // otherwise we use trigonmetry to calculate the angle
        if(diffVect.magnitude != 0)
        {
            angle = 2 * Mathf.Asin((diffVect.magnitude)/2);
        }
        else
        {
            angle = 0;
        }
        
        // our previous calculation does not account for the quadrant the angle is in
        // so we need to check the sign of the x component of the difference vector
        // and adjust the angle accordingly
        if(diffVect.x < 0)
        {
            angle = (2 * Mathf.PI) - angle;
        }

    }
    // this is called once per frame
    void Update()
    {
        // we use coroutines here to avoid putting all the logic for
        // changing the colour and speed of the cube
        // directly in the update function
        // so it can be executed over multiple frames


        // only start a coroutine again, if it has finished from last time
        if (!collectingHeartRates)
        {
            StartCoroutine(AverageHeartRate());         
        }
        if (!changingColour)
        {
            StartCoroutine(ChangeColour());
        }


        if (!collectingMeditation)
        {
            StartCoroutine(AverageMeditation());
        }

        if (!changingVelocity)
        {
            StartCoroutine(ChangeVelocity());
        }
    }
     void FixedUpdate()
     {
        // the code here is called to update the
        // position at which the cubes are
        
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

            // pauses, and then resumes in the following frame
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

            // pauses, and then resumes in the following frame
            yield return null;
        }

        averageMeditation = Mathf.RoundToInt((float)meditationAverages.Average());
    }

    private float HeartRateSigmoid(int hr, int midpoint = 70, int scale = 5)
    {
        // based on the sigmoid function, but higher values correspond to
        // smaller y value
        float x = (hr - midpoint) / scale;
        float sigmoid = 1 / (1 + Mathf.Exp(x));
        return sigmoid;
    }

    private float MeditationLinear(int med, float floor=0.1f, float ceiling=1f)
    {
        // Linear relationship between the speed of the cubes and the meditation value
        // with a floor and ceiling (to stop the cubes going to quick or slow)
        float delta = ceiling- floor;
        float y = ceiling - (delta * ((float)med / 100));
        return y;

    }
    public static void ChangeBaseHR(int newBase)
    {
        // ensure the base heart rate is in a suitable range
        if(newBase >30 && newBase < 150)
        {
            baseLineHR = newBase;
        }

    }
    IEnumerator ChangeColour()
    {
        // ensures that the courtine is not called again untill it completes
        changingColour = true;
        Debug.Log(Time.deltaTime + " HR Average:" + averageHeartRate);
        float heartRateScaled = HeartRateSigmoid(averageHeartRate, baseLineHR);
        Debug.Log("Baseline cube " + baseLineHR);



        Color oldColour = cubeRenderer.material.color;

        // varies between white (low HR's) and black (high HR's)
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
        // at the moment this will just make it vary between slow (high meditation value) and faster (low meditation value)
        float newAngularVelocity = meditationScaled;

        float time = 0f;

        while (time < refreshDuration)
        {
            // linearly interpolate between old and new angular velocity
            angularVelocity = Mathf.Lerp(oldAngularVelocity,newAngularVelocity, time / refreshDuration);
            time += Time.deltaTime;

            yield return null;
        }

        angularVelocity = newAngularVelocity;
        changingVelocity = false;
    }
}