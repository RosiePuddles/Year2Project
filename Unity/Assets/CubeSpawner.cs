using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CubeSpawner : MonoBehaviour
{
    // Reference to the Prefab. Drag a Prefab into this field in the Inspector.
    public GameObject cube;

    [SerializeField]
    public static int cubeRadius { get; private set; } = 20;
    [SerializeField]
    int cubeCount = 5;

    float x;
    float y;
    float z;

    // This script will simply instantiate the Prefab when the game starts.
    void Awake()
    {
        // Instantiate at position (0, 0, 0) and zero rotation.
        float currentAngle = 0;
        float angleIncrement = (2*Mathf.PI)/cubeCount;
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
