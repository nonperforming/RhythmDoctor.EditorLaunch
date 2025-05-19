global using System;
global using System.IO;
global using BepInEx;
global using BepInEx.Logging;
global using RDLevelEditor;
global using HarmonyLib;
global using UnityEngine;

namespace RhythmDoctor.EditorLaunch;

[BepInPlugin(MyPluginInfo.PLUGIN_GUID, MyPluginInfo.PLUGIN_NAME, MyPluginInfo.PLUGIN_VERSION)]
[BepInProcess("Rhythm Doctor.exe")]
internal class Plugin : BaseUnityPlugin
{
  private static new ManualLogSource Logger;
  private static readonly string EditorLaunchFile = Path.Join(Application.persistentDataPath, "editorlaunch.txt");

  private ConfigEntry<bool> ConfigRelaunchSteam;

  internal static string Level;

  private void Awake()
  {
    Logger = base.Logger;
    Logger.LogInfo($"Plugin {MyPluginInfo.PLUGIN_GUID} is loaded!");

    ConfigRelaunchSteam = Config.Bind("General", "RelaunchInSteam", true, "Relaunch the game with Steam to track playtime");
    
    RDStartup.Setup();
    if (!LaunchedWithLevel()) return;
    if (ConfigRelaunchSteam.Value && !SteamIntegration.initialized)
    {
      Logger.LogWarning("Not launched with Steam, relaunching");
      File.WriteAllText(EditorLaunchFile, Level);
      Application.OpenURL("steam://launch/774181");
      Application.Quit();
    }

    Harmony.CreateAndPatchAll(typeof(Patch), MyPluginInfo.PLUGIN_GUID);
    scnBase.GoToScene(nameof(scnEditor));
  }

  private static bool IsValidLevel(string path)
  {
    Logger.LogDebug($"Checking {path}");
    if (Path.GetExtension(path) is ".rdlevel" or ".rdzip")
    {
      Logger.LogDebug("Extension check passed");
      if (File.Exists(path))
      {
        Logger.LogDebug("Exist check passed");
        return true;
      }
      Logger.LogDebug("Exist check failed");
    }
    else
    {
      Logger.LogDebug("Extension check failed");
    }

    return false;
  }

  private static bool LaunchedWithLevel()
  {
    if (File.Exists(EditorLaunchFile))
    {
      string path = File.ReadAllText(EditorLaunchFile);
      File.Delete(EditorLaunchFile);
      Logger.LogInfo($"Editor launch file found: {path}");

      if (IsValidLevel(path))
      {
        Level = path;
        return true;
      }
      Logger.LogError("Editor launch file is bogus");
    }
    else
    {
      Logger.LogInfo("Editor launch file not found");
    }

    string[] args = Environment.GetCommandLineArgs();
    if (args.Length == 1 || (args.Length == 2 && args[1] == "--force-d3d11"))
    {
      Logger.LogInfo("No file passed");
      return false;
    }

    foreach (string argument in args)
    {
      if (!IsValidLevel(argument)) continue;

      Level = argument;
      return true;
    }

    Logger.LogInfo("Could not find level");
    return false;
  }
}