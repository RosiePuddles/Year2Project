using Unity.VisualScripting;
using UnityEngine;

public class Rot : MonoBehaviour
{
    public float speed;
    [Header("Colour 1")]
    public Color Colour1;
    public AnimationCurve Brightness1;
    [Header("Colour 2")]
    public Color Colour2;
    public AnimationCurve Brightness2;
    [Header("Colour 3")]
    public Color Colour3;
    public AnimationCurve Brightness3;
    [Header("Colour 4")]
    public Color Colour4;
    public AnimationCurve Brightness4;
    private Material skybox;
    private int[] colIDs;
    private float[] alphas;

    private void Start()
    {
        skybox = GetComponent<Skybox>().material;
        colIDs = new[] { Shader.PropertyToID("_Colour_1"), Shader.PropertyToID("_Colour_2"), Shader.PropertyToID("_Colour_3"), Shader.PropertyToID("_Colour_4") };
        alphas = new[] { Colour1.a, Colour2.a, Colour3.a, Colour4.a };
    }

    // Update is called once per frame
    void Update()
    {
        skybox.SetColor(colIDs[0], new Color(
            Colour1.r * (1f + Brightness1.Evaluate(Time.time)),
            Colour1.g * (1f + Brightness1.Evaluate(Time.time)),
            Colour1.b * (1f + Brightness1.Evaluate(Time.time)), alphas[0]));
        skybox.SetColor(colIDs[1], new Color(
            Colour2.r * (1f + Brightness2.Evaluate(Time.time)),
            Colour2.g * (1f + Brightness2.Evaluate(Time.time)),
            Colour2.b * (1f + Brightness2.Evaluate(Time.time)), alphas[1]));
        skybox.SetColor(colIDs[2], new Color(
            Colour3.r * (1f + Brightness3.Evaluate(Time.time)),
            Colour3.g * (1f + Brightness3.Evaluate(Time.time)),
            Colour3.b * (1f + Brightness3.Evaluate(Time.time)), alphas[2]));
        skybox.SetColor(colIDs[3], new Color(
            Colour4.r * (1f + Brightness4.Evaluate(Time.time)),
            Colour4.g * (1f + Brightness4.Evaluate(Time.time)),
            Colour4.b * (1f + Brightness4.Evaluate(Time.time)), alphas[3]));
        // Color newCol = Color.HSVToRGB(h + Brightness1.Evaluate(Time.time * 2 + 5f) / 20f, s, v + (Mathf.Sin(Time.time) + 1f) / 10f);
        // newCol.a = 0.21f - (Mathf.Sin(Time.time * 2 + 5f) + 1f) / 15f;
        // skybox.SetColor(colIDs[0], newCol);
        transform.Rotate(0f, speed * Time.deltaTime, 0f);
    }
}
