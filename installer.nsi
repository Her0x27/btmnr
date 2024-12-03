!define PRODUCT_NAME "Bluetooth Headphones Manager"
!define PRODUCT_PUBLISHER "CapMustDie"
!define PRODUCT_WEB_SITE "https://github.com/Her0x27/btmnr/"
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\btmnr.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"

SetCompressor lzma

Name "${PRODUCT_NAME}"
OutFile "btmnr-setup.exe"
InstallDir "$PROGRAMFILES\${PRODUCT_NAME}"
InstallDirRegKey HKLM "${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show

Section "MainSection" SEC01
  SetOutPath "$INSTDIR"
  SetOverwrite ifnewer
  File "installer\btmnr.exe"
  File "installer\config.json"
  File "installer\README.md"
  
  CreateDirectory "$SMPROGRAMS\${PRODUCT_NAME}"
  
  # Install Windows Service
  ExecWait 'sc create BluetoothManager binPath= "$INSTDIR\btmnr.exe" start= auto'
  ExecWait 'sc description BluetoothManager "Manages Bluetooth headphones connection based on audio activity"'
  ExecWait 'sc start BluetoothManager'
SectionEnd

Section -Post
  WriteUninstaller "$INSTDIR\uninst.exe"
  WriteRegStr HKLM "${PRODUCT_DIR_REGKEY}" "" "$INSTDIR\btmnr.exe"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "DisplayName" "$(^Name)"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninst.exe"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\btmnr.exe"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
SectionEnd

Section Uninstall
  # Stop and remove service
  ExecWait 'sc stop BluetoothManager'
  ExecWait 'sc delete BluetoothManager'
  
  Delete "$INSTDIR\btmnr.exe"
  Delete "$INSTDIR\config.json"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\uninst.exe"
  
  Delete "$SMPROGRAMS\${PRODUCT_NAME}\${PRODUCT_NAME}.lnk"
  RMDir "$SMPROGRAMS\${PRODUCT_NAME}"
  RMDir "$INSTDIR"
  
  DeleteRegKey HKLM "${PRODUCT_UNINST_KEY}"
  DeleteRegKey HKLM "${PRODUCT_DIR_REGKEY}"
SectionEnd
