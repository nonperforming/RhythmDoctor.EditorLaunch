namespace RhythmDoctor.EditorLaunch;

[HarmonyPatch(typeof(scnEditor))]
internal static class Patch
{
  [HarmonyPatch(nameof(scnEditor.LoadDefaultLevel))]
  [HarmonyPostfix]
  static void DoNotLoadDefaultLevel(ref bool __runOriginal)
  {
    __runOriginal = false;
  }

  [HarmonyPatch(nameof(scnEditor.Start))]
  [HarmonyPostfix]
  static void LoadCustomLevel()
  {
    switch (Path.GetExtension(Plugin.Level))
    {
      case ".rdlevel":
        scnEditor.instance.OpenFileWithPath(Plugin.Level);
        break;
      case ".rdzip":
      case ".zip":
        scnEditor.instance.StartCoroutine(scnEditor.instance.OpenLevelPackage(Plugin.Level));
        break;
    }

    Harmony.UnpatchID(MyPluginInfo.PLUGIN_GUID);
  }
}