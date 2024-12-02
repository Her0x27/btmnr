# Bluetooth Headphones Manager (btmnr)
* Windows service that automatically manages Bluetooth headphones based on audio activity
* Системный сервис Windows для автоматического управления Bluetooth-наушниками на основе активности воспроизведения звука.

## Возможности
- Автоматическое отключение наушников после периода неактивности
- Автоматическое подключение при начале воспроизведения звука
- Настраиваемая конфигурация через JSON файл
- Работает как служба Windows
- Ведение журнала событий

## Установка
1. Скачайте последнюю версию из раздела Releases
2. Распакуйте архив в желаемую директорию
3. Настройте config.json под ваши наушники
4. Установите службу через PowerShell с правами администратора:
```powershell
sc create BluetoothManager binPath= "полный\путь\к\btmnr.exe"
sc start BluetoothManager
```

## Конфигурация
Файл config.json содержит следующие настройки:
```
{
    "inactivity_timeout": 300,    // Время в секундах до отключения при отсутствии звука
    "auto_connect": true,         // Автоматическое подключение при начале воспроизведения
    "device_address": "XX:XX:XX:XX:XX:XX"  // MAC-адрес ваших наушников
}
```

## Как узнать MAC-адрес наушников
    Откройте Параметры Windows → Bluetooth и устройства
    Найдите ваши наушники
    Нажмите на "..." → Свойства
    MAC-адрес указан в разделе "Сведения об устройстве"

## Журнал событий
## Журнал работы сервиса находится в файле bluetooth_manager.log в директории установки.

## Системные требования
- Windows 10/11
- Bluetooth-адаптер
- Права администратора для установки службы

## Разработка
## Структура проекта:
```
btmnr/
├── .github/
│   └── workflows/
│       └── build.yml
├── src/
│   ├── main.rs
│   ├── audio.rs
│   ├── bluetooth.rs
│   └── config.rs
├── Cargo.toml
├── config.json
├── README.md
└── .gitignore
```

## Для сборки из исходного кода:

```cargo build --release```

## Лицензия
MIT License
