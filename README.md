# Honkai: Star Rail Dumper [![License](https://img.shields.io/badge/License-GPL3.0-green.svg)](https://github.com/lanylow/honkai-dumper/blob/main/LICENSE) [![Discord](https://img.shields.io/badge/chat-discord-informational)](https://discord.gg/MrtJvV5tKv)

Free and open-source tool for Honkai: Star Rail that allows you to dump all classes, fields and methods that comprise the game's code. The program is developed as an internal module that's being loaded into the game process and it's compatible only with the global version of the game.

## FAQ

### What does this tool generate?
Two output formats are supported, one writes everything to a C# source file, while the other one extracts only the methods and saves them to a JSON file. You can choose which one to use by modifying the `base.rs` file.

### How to load the tool?
I am not providing any builds of this program, so if you want to try it out you will have to compile it yourself. Once you do that, you can use the loader from my other program, [Genshin Utility](https://github.com/lanylow/genshin-utility), to load this tool into the game.

### How do I know if it worked?
About 10 seconds after injecting the tool into the game a console should show up. Once you see a message saying `done` in it, a file called `methods.json` or `dump.cs` should appear in the same folder where the game (`StarRail.exe`) is installed.

### Why are some methods just random characters?
Many methods inside the game are obfuscated by developers in order to make reverse engineering harder. There is very little you can do about it.

## Example output

Here are some snippets from files generated with this tool.

The C# source file:
```cs
// Namespace: RPG.Client
public class AutoScrollRect : ScrollRect
{
	// Fields
	private UnityEngine.Vector3[] KOIAJFFGPGF; // 0x180

	// Methods
	// RVA: 0x1febdf0 VA: 0x181febdf0
	public void .ctor() { }

	// RVA: 0x1feb7a0 VA: 0x181feb7a0
	protected void POFLDHKMLAF() { }

	// RVA: 0x1feb7f0 VA: 0x181feb7f0
	public void SetItemSelectCallback() { }

	// RVA: 0x1feb700 VA: 0x181feb700
	private void MNOGNALMGPL(RPG.Client.AnimatorButton) { }

	// RVA: 0x1feb9c0 VA: 0x181feb9c0
	public void SnapTo(UnityEngine.GameObject) { }
}
```

The JSON file which contains only the methods:
```json
{
	"RPG.Client.AutoScrollRect::.ctor": "0x1febdf0",
	"RPG.Client.AutoScrollRect::MNOGNALMGPL": "0x1feb700",
	"RPG.Client.AutoScrollRect::POFLDHKMLAF": "0x1feb7a0",
	"RPG.Client.AutoScrollRect::SetItemSelectCallback": "0x1feb7f0",
	"RPG.Client.AutoScrollRect::SnapTo": "0x1feb9c0",
}
```

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](https://github.com/lanylow/honkai-dumper/blob/master/LICENSE) file for details.
