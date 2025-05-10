using System;
using System.IO;
using BepInEx;
using BepInEx.Logging;
using HarmonyLib;
using RDLevelEditor;
using UnityEngine;

namespace RhythmDoctor.EditorLaunch;

[BepInPlugin(MyPluginInfo.PLUGIN_GUID, MyPluginInfo.PLUGIN_NAME, MyPluginInfo.PLUGIN_VERSION)]
[BepInProcess("Rhythm Doctor.exe")]
public class Plugin : BaseUnityPlugin
{
  internal static new ManualLogSource Logger;
  internal static readonly string EditorLaunchFile = Path.Join(Application.persistentDataPath, "editorlaunch.txt");

  internal static string Level = null;
  internal static bool RunOnce;

  private void Awake()
  {
    // Plugin startup logic
    Logger = base.Logger;
    Logger.LogInfo($"Plugin {MyPluginInfo.PLUGIN_GUID} is loaded!");

    Harmony.CreateAndPatchAll(typeof(Patch), "non.editorlaunch");
  }

  internal static bool LaunchedWithLevel()
  {
    if (File.Exists(EditorLaunchFile))
    {
      Level = File.ReadAllText(EditorLaunchFile);
      File.Delete(EditorLaunchFile);
      Logger.LogInfo($"Editor launch file found: {Level}");
      
      string extension = Path.GetExtension(Path.GetFileName(Level));
      Logger.LogDebug($"Extension is {extension}");
      if (extension == ".rdlevel" || extension == ".rdzip")
      {
        if (File.Exists(Level))
        {
          return true;
        }
      }
      Logger.LogInfo("Editor launch file is bogus");
      Level = null;
    }
    else
    {
      Logger.LogInfo("Editor launch file not found");
    }

    string[] args = Environment.GetCommandLineArgs();
    if (args.Length == 1)
    {
      Logger.LogInfo("No file passed");
      return false;
    }

    foreach (string argument in Environment.GetCommandLineArgs())
    {
      string extension = Path.GetExtension(Path.GetFileName(argument));

      Logger.LogDebug($"Extension of {argument} is {extension}");
      if (extension == ".rdlevel" || extension == ".rdzip")
      {
        Logger.LogDebug("Extension check passed");
        if (File.Exists(argument))
        {
          Logger.LogDebug("Exist check passed");
          Level = argument;
        }
        else
        {
          Logger.LogDebug("Exist check failed");
        }
      }
      else
      {
        Logger.LogDebug("Extension check failed");
      }
    }

    if (Level.IsNullOrWhiteSpace())
    {
      Logger.LogInfo("Couldn't find a level");
      return false;
    }

    Logger.LogInfo($"Launched with {Level}");
    return true;
  }

  [HarmonyPatch(typeof(RDStartup))]
  class Patch
  {
    [HarmonyPatch(nameof(RDStartup.Setup))]
    [HarmonyPostfix]
    static void PiggybackSetup()
    {
      if (RunOnce) return;
      if (!LaunchedWithLevel()) return;
      
      RunOnce = true;
      
      // Check if Steam is open
      if (!SteamIntegration.initialized)
      {
        Logger.LogWarning("Not launched with Steam, relaunching");
        File.WriteAllText(EditorLaunchFile, Level);
        Application.OpenURL("steam://launch/774181");
        Application.Quit();
      }

      scnCLS.customLevelPath = Level;
      scnBase.GoToScene(nameof(scnEditor));
    }
  }
}