; Script generated for iTools
; Inno Setup Compiler 6.0+

[Setup]
AppName=iTools
AppVersion=0.1.0
AppPublisher=Soft
DefaultDirName=D:\Program Files\iTools
DefaultGroupName=iTools
UninstallDisplayIcon={app}\itools.exe
OutputDir=D:\03Work\01pack_setup\iToolsPack
OutputBaseFilename=iTools_Setup_0.1.0

Compression=lzma2
SolidCompression=yes
PrivilegesRequired=lowest
; 静默安装关键设置
DisableDirPage=auto
DisableProgramGroupPage=auto
DisableReadyPage=yes
DisableFinishedPage=yes

; 不询问用户是否重启
RestartIfNeededByRun=no
;安装包文件的图标
SetupIconFile=D:\03Work\01pack_setup\iTools\icons\icon.ico

[Files]
; 复制整个 iTools 目录下的所有文件及子文件夹
Source: "D:\03Work\01pack_setup\iTools\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
; 创建桌面快捷方式
Name: "{userdesktop}\iTools"; Filename: "{app}\itools.exe"; WorkingDir: "{app}"; IconFilename: "{app}\icons\icon.ico"
; 创建开始菜单快捷方式
Name: "{userstartmenu}\iTools\iTools"; Filename: "{app}\itools.exe"; WorkingDir: "{app}"; IconFilename: "{app}\icons\icon.ico"
; 卸载快捷方式（可选）
Name: "{userstartmenu}\iTools\Uninstall iTools"; Filename: "{uninstallexe}"

[Run]
; 安装完成后不自动执行任何程序，所以留空

[UninstallDelete]
; 卸载时删除整个安装文件夹（谨慎使用）
Type: filesandordirs; Name: "{app}"

[Code]
// 可选：在静默安装时抑制任何弹窗（增强静默效果）
function InitializeSetup(): Boolean;
begin
  // 如果是静默模式（/SILENT 或 /VERYSILENT），禁用所有提示
  if WizardSilent then
  begin
    Result := True;
  end
  else
    Result := True;
end;
