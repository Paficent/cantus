import type { LogEntry, LogLevel } from "$lib/types/log";

const LOG_PATTERN = /^\[([^\]]+)\]\s+\[(\w+)\]\s+\[([^\]]+)\]\s+(.*)$/;

export function parseLine(line: string): LogEntry | null {
  const match = line.match(LOG_PATTERN);
  if (!match) return null;
  return {
    timestamp: match[1],
    level: match[2] as LogLevel,
    source: match[3],
    message: match[4],
  };
}

export function parseLog(text: string): LogEntry[] {
  return text
    .split("\n")
    .filter((l) => l.trim())
    .map(parseLine)
    .filter((e): e is LogEntry => e !== null);
}

const RAW = `[22:53:59.981] [info] [main] jeode initializing (gameDir='C:\\Program Files (x86)\\Steam\\steamapps\\common\\My Singing Monsters')
[22:53:59.984] [debug] [hooks] MinHook initialized
[22:53:59.984] [debug] [file_hook] resolved fopen=0x76cf9580, _wfopen=0x76cf9b70, _wfopen_s=0x76cf87b0
[22:54:00.183] [debug] [file_hook] hooks installed (passthrough until configured)
[22:54:00.212] [debug] [sched] found via pattern at 0x011ac680
[22:54:00.278] [info] [sched] hook installed at 0x11ac680
[22:54:00.346] [info] [egl] hook installed at 0x6c612080
[22:54:00.346] [info] [hooks] early hooks installed (file=true, scheduler=true, egl=true)
[22:54:00.347] [debug] [main] mods directory: 'C:\\Program Files (x86)\\Steam\\steamapps\\common\\My Singing Monsters\\mods' (exists=true)
[22:54:00.348] [debug] [loader] scanning 'C:\\Program Files (x86)\\Steam\\steamapps\\common\\My Singing Monsters\\mods'
[22:54:00.350] [info] [loader] scanned 0 directories, loaded 0 mod(s)
[22:54:00.350] [debug] [loader] resolving dependencies for 0 mod(s)
[22:54:00.350] [info] [loader] load order:
[22:54:00.350] [debug] [loader] global overrides built: 0 asset, 0 dat
[22:54:00.350] [info] [main] 0 mod(s) loaded, 0 asset override(s), 0 dat override(s)
[22:54:00.351] [debug] [game_lua] .text section at 0x00fa1000 (size 0xd7edda)
[22:54:00.353] [info] [game_lua] luaL_loadbuffer resolved via pattern at 0x0110ac80
[22:54:00.355] [info] [game_lua] lua_pcall resolved via pattern at 0x011077a0
[22:54:00.357] [info] [game_lua] lua_settop resolved via pattern at 0x01107010
[22:54:00.359] [info] [game_lua] lua_newthread resolved via pattern at 0x01106f80
[22:54:00.362] [info] [game_lua] lua_resume resolved via pattern at 0x01192190
[22:54:00.364] [info] [game_lua] lua_yield resolved via pattern at 0x01192630
[22:54:00.364] [info] [game_lua] all 6 lua functions resolved
[22:54:00.366] [debug] [file_hook] data base: 'c:\\program files (x86)\\steam\\steamapps\\common\\my singing monsters\\data\\'
[22:54:00.366] [debug] [file_hook] dat base: 'c:\\users\\lukef\\appdata\\locallow\\big blue bubble inc\\my singing monsters\\1\\'
[22:54:00.366] [info] [file_hook] active with 0 asset + 0 dat override(s)
[22:54:00.366] [debug] [hooks] found luaopen_game at 0x1574730
[22:54:00.426] [info] [hooks] luaopen_game hook active
[22:54:00.426] [debug] [egl] overlays enabled, toggle key 0x70
[22:54:00.426] [debug] [hooks] EGL configured (overlays=true, toggle=0x70)
[22:54:00.426] [info] [hooks] hook setup complete
[22:54:00.426] [info] [main] initialization complete
[22:54:00.808] [debug] [exception] code=0x40010006 at address=0x752c1e54
[22:54:00.809] [debug] [exception] EAX=0x008FDBC0 EBX=0x00000000 ECX=0x00000002 EDX=0x00000000
[22:54:00.809] [debug] [exception] ESI=0x008FE2A8 EDI=0x6CC6E3A0 EBP=0x008FDC18 ESP=0x008FDBC0
[22:54:00.809] [debug] [exception] EIP=0x752C1E54
[22:54:09.207] [debug] [exception] code=0xE06D7363 at address=0x752c1e54
[22:54:09.207] [debug] [exception] EAX=0x008FE958 EBX=0x19930520 ECX=0x00000003 EDX=0x00000000
[22:54:09.207] [debug] [exception] EIP=0x752C1E54
[22:54:09.371] [info] [egl] overlay initialized (hwnd=0x60622)
[22:54:09.430] [debug] [hooks] hooked_luaopen_game called, L=0x11ec97a8
[22:54:09.476] [debug] [hooks] lua thread state captured
[22:54:11.146] [debug] [file_hook] loading screen texture detected, arming game-ready trigger
[22:54:11.146] [debug] [file_hook] game ready trigger fired on 'gfx/menu/bbb_logo_loading_screen.pvr.gz'
[22:54:11.146] [info] [hooks] game ready, preparing mod environment
[22:54:11.146] [debug] [hooks] queuing native mod load (nativeEnabled=false) and environment init
[22:54:11.414] [debug] [sched] draining 2 work item(s)
[22:54:11.416] [debug] [native] processing 0 mod(s) (native enabled=false)
[22:54:11.424] [debug] [file_api] initialized (gameDir='C:/Program Files (x86)/Steam/steamapps/common/My Singing Monsters')
[22:54:11.425] [debug] [api] Lua api registered
[22:54:11.425] [debug] [env] APIs registered (gameDir='C:/Program Files (x86)/Steam/steamapps/common/My Singing Monsters')
[22:54:11.425] [info] [env] game version: '5.3.2', 0 mod(s) to load
[22:54:11.425] [info] [env] environment initialization complete
[22:54:14.694] [debug] [exception] code=0xE06D7363 at address=0x752c1e54
[22:54:14.694] [debug] [exception] EIP=0x752C1E54
[22:54:16.202] [debug] [exception] code=0xE06D7363 at address=0x752c1e54
[22:54:16.203] [debug] [exception] EIP=0x752C1E54
[22:54:17.154] [debug] [exception] code=0xE06D7363 at address=0x752c1e54
[22:54:17.154] [debug] [exception] EIP=0x752C1E54
[22:54:35.437] [debug] [exception] code=0xE06D7363 at address=0x752c1e54
[22:54:36.120] [debug] [exception] code=0xE06D7363 at address=0x752c1e54
[22:54:38.738] [debug] [exception] code=0xE06D7363 at address=0x752c1e54
[22:54:52.084] [debug] [exception] code=0xE06D7363 at address=0x752c1e54
[22:54:52.084] [debug] [exception] EIP=0x752C1E54
[22:59:05.058] [info] [main] libjeode unloading
[22:59:05.060] [warn] [hooks] unhooking with active mods (forced shutdown)
[22:59:05.061] [error] [native] failed to unload better-login.dll: module still in use
[22:59:05.062] [info] [main] cleanup complete`;

export const MOCK_LOGS = parseLog(RAW);
