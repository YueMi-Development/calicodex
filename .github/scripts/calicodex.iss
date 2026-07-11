[Setup]
AppName=CalicoDex
AppVersion={#AppVersion}
DefaultDirName={autopf}\CalicoDex
DefaultGroupName=CalicoDex
UninstallDisplayIcon={app}\calicodex.exe
Compression=lzma2
SolidCompression=yes
OutputDir={#OutputDir}
OutputBaseFilename={#OutputBaseFilename}
ChangesEnvironment=yes
PrivilegesRequired=admin

[Files]
Source: "{#StageDir}\calicodex.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#StageDir}\codex-code-mode-host.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#StageDir}\codex-responses-api-proxy.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#StageDir}\codex-app-server.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#StageDir}\codex-command-runner.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#StageDir}\codex-windows-sandbox-setup.exe"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
    ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; \
    Flags: preservestringtype; Check: NeedsAddPath(ExpandConstant('{app}'))

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
  ParamUpper: string;
  OrigPathUpper: string;
begin
  if RegQueryStringValue(HKLM, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', OrigPath) then
  begin
    ParamUpper := UpperCase(Param);
    OrigPathUpper := UpperCase(OrigPath);
    Result := (Pos(';' + ParamUpper + ';', ';' + OrigPathUpper + ';') = 0) and
              (Pos(';' + ParamUpper, ';' + OrigPathUpper) = 0);
  end
  else
  begin
    Result := True;
  end;
end;
