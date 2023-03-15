using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CubeSpawner : MonoBehaviour
{
    public GameObject cube;

    [SerializeField]
    public static int cubeRadius { get; private set; } = 20;
    [SerializeField]
    int cubeCount = 5;

    float x;
    float y;
    float z;

    // Instantiates the Prefab cubes when the game starts.
    void Awake()
    {
        // Instantiates at position (0, 0, 0) and zero rotation.
        float currentAngle = 0;

        // the difference in radians, between each cubes position
        float angleIncrement = (2*Mathf.PI)/cubeCount;

        // instantiate the cubes so they are evenly spaced around a unit circle
        for (int i = 0; i < cubeCount; i++)
        {
            x = Mathf.Sin(currentAngle);
            y = 0f;
            z = Mathf.Cos(currentAngle);
            Instantiate(cube, new Vector3(x, y, z), Quaternion.identity);
            currentAngle += angleIncrement;
        }

    }

}
