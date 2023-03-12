using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.XR;

public class PauseMenu : MonoBehaviour
{
    private GameObject pause_menu;
    private GameObject left_controller;
    private GameObject right_controller;

    private List<InputDevice> inputDevices = new List<InputDevice>();
    private bool menu_pressed = false;
    void Start()
    {
        pause_menu = GameObject.Find("PauseMenu");
        pause_menu.SetActive(false);

        left_controller = GameObject.Find("LeftHand Controller");
        left_controller.SetActive(false);

        right_controller = GameObject.Find("RightHand Controller");
        right_controller.SetActive(false);

        UnityEngine.XR.InputDevices.GetDevices(inputDevices);
    }
    void Update()
    {
        foreach (var device in inputDevices)
        {
            bool menu_button;
            if (device.TryGetFeatureValue(CommonUsages.menuButton, out menu_button) && menu_button && !menu_pressed)
            {
                menu_pressed = true;
                pause_menu.SetActive(!pause_menu.activeSelf);
                left_controller.SetActive(!left_controller.activeSelf);
                right_controller.SetActive(!right_controller.activeSelf);
                StartCoroutine(WaitForRelease(device));
                
            }
        }
    }
    private IEnumerator WaitForRelease(InputDevice device)
    {
        bool menu_button;
        while (device.TryGetFeatureValue(CommonUsages.menuButton, out menu_button) && menu_button)
        {
            yield return new WaitForSeconds(0.01f);
        }
        menu_pressed = false;
    }
}
