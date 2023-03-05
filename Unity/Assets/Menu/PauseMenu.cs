using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class PauseMenu : MonoBehaviour
{
    private GameObject pause_menu;
    void Start()
    {
        pause_menu = GameObject.Find("PauseMenu");
        pause_menu.SetActive(false);
    }
    void Update()
    {
        if (Input.GetKeyDown("space"))
        {
            pause_menu.SetActive(true);
        }
    }
}
