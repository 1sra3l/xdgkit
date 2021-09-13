#!/bin/bash

LIST=("Building" "Debugger" "IDE" "GUIDesigner" "Profiling" "RevisionControl" "Translation" "Calendar" "ContactManagement" "Database" "Dictionary" "Chart" "Email" "Finance" "FlowChart" "PDA" "ProjectManagement" "Presentation" "Spreadsheet" "WordProcessor" "Graphics2D" "VectorGraphics" "RasterGraphics" "Graphics3D" "Scanning" "OCR" "Photography" "Publishing" "Viewer" "TextTools" "DesktopSettings" "HardwareSettings" "Printing" "PackageManager" "Dialup" "InstantMessaging" "Chat" "IRCClient" "Feed" "FileTransfer" "HamRadio" "News" "P2P" "RemoteAccess" "Telephony" "TelephonyTools" "VideoConference" "WebBrowser" "WebDevelopment" "Midi" "Mixer" "Sequencer" "Tuner" "TV" "AudioVideoEditing" "VideoPlayer" "Recorder" "DiscBurning" "ActionGame" "AdventureGame" "ArcadeGame" "BoardGame" "BlocksGame" "CardGame" "KidsGame" "LogicGame" "RolePlaying" "Shooter" "Simulation" "SportsGame" "StrategyGame" "Art" "Construction" "Music" "Languages" "ArtificialIntelligence" "Astronomy" "Biology" "Chemistry" "ComputerScience" "DataVisualization" "Economy" "Electricity" "Geography" "Geology" "Geoscience" "History" "Humanities" "ImageProcessing" "Literature" "Maps" "Math" "NumericalAnalysis" "MedicalSoftware" "Physics" "Robotics" "Spirituality" "Sports" "ParallelComputing" "Amusement" "Archiving" "Compression" "Electronics" "Emulator" "Engineering" "FileTools" "FileManager" "TerminalEmulator" "Filesystem" "Monitor" "Security" "Accessibility" "Calculator" "Clock" "TextEditor" "Documentation" "Adult" "Core" "KDE" "GNOME" "XFCE" "GTK" "Qt" "Motif" "Java" "ConsoleOnly" "Unknown")
TOP="impl AdditionalCategories {
    pub fn from_string(item:String) ->  AdditionalCategories {
        "
BOTTOM="impl fmt::Display for AdditionalCategories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String = match *self {"
for item in ${LIST[@]}
do
  if [[ $item != "Unknown" ]]
  then
    TOP="${TOP}if item == \"$item\" {
            return AdditionalCategories::$item
        } else "
  else
    if [[ $item != "Graphics2D" ]] && [[ $item != "Graphics3D" ]]
    then
    TOP="${TOP} {
            return AdditionalCategories::$item"
    fi
  fi
  if [[ $item != "Graphics2D" ]] && [[ $item != "Graphics3D" ]]
  then
    BOTTOM="${BOTTOM}
            AdditionalCategories::$item => \"$item\".to_string(),"
  fi
  if [[ $item == "Graphics2D" ]]
  then
    TOP="${TOP}if item == \"2DGraphics\" {
            return AdditionalCategories::$item
        } else "
    BOTTOM="${BOTTOM}
            AdditionalCategories::$item => \"2DGraphics\".to_string(),"
  fi
  if [[ $item == "Graphics3D" ]]
  then
    TOP="${TOP}if item == \"3DGraphics\" {
            return AdditionalCategories::$item
        } else "
    BOTTOM="${BOTTOM}
            AdditionalCategories::$item => \"3DGraphics\".to_string(),"
  fi
done
TOP="${TOP}
        }
        AdditionalCategories::Unknown
    }
}"
BOTTOM="${BOTTOM}
        };
        write!(f, \"{:?}\", v)
    }
}
"
echo "${TOP}
${BOTTOM}"
