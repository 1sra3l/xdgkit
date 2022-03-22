/*!
# Categories

The rust enum-ification of the XDG Desktop Entry Categories specifications.

This comes complete with `Categories::default()` as well as implementing `Display`
`from_string()` is also implemented

*/

// categories.rs
// Rusified in 2021 Copyright Israel Dahl. All rights reserved.
// 
//        /VVVV\
//      /V      V\
//    /V          V\
//   /      0 0     \
//   \|\|\</\/\>/|/|/
//        \_/\_/
// 
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

use std::fmt;
use serde::{Deserialize, Serialize};
#[allow(dead_code, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum Categories {
    /// Application for presenting, creating, or processing multimedia (audio/video)
    AudioVideo,
    /// An audio application **Desktop entry must include AudioVideo as well**
    Audio,
    /// A video application **Desktop entry must include AudioVideo as well Development An application for development**
    Video,
    /// Educational software
    Education,
    /// A game
    Game,
    /// Application for viewing, creating, or processing graphics
    Graphics,
    /// Network application such as a web browser
    Network,
    /// An office type application
    Office,
    /// Scientific software
    Science,
    /// Settings applications **Entries may appear in a separate menu or as part of a "Control Center"**
    Settings,
    /// System application, "System Tools" such as say a log viewer or network monitor
    System,
    /// Small utility application, "Accessories"
    Utility,

// ADDITIONAL CATEGORIES ---------------------------------------------------------------------------------------------------------------------
    /// A tool to build applications `Development`
    Building,
    /// A tool to debug applications `Development`
    Debugger,
    /// IDE application Development
    IDE,
    /// A GUI designer application `Development`
    GUIDesigner,
    /// A profiling tool `Development`
    Profiling,
    /// Applications like cvs or subversion `Development` 
    RevisionControl,
    /// A translation tool `Development`
    Translation,
    /// Calendar application `Office`
    Calendar,
    /// E.g. an address book `Office`
    ContactManagement,
    /// Application to manage a database `Office` or `Development` or `AudioVideo`
    Database,
    /// A dictionary `Office` or `TextTools`
    Dictionary,
    /// Chart application `Office`
    Chart,
    /// Email application `Office` or `Network`
    Email,
    /// Application to manage your finance `Office`
    Finance,
    /// A flowchart application `Office`
    FlowChart,
    /// Tool to manage your PDA `Office`
    PDA,
    /// Project management application `Office` or `Development`
    ProjectManagement,
    /// Presentation software `Office`
    Presentation,
    /// A spreadsheet `Office`
    Spreadsheet,
    /// A word processor `Office`
    WordProcessor,
    /// 2D based graphical application `Graphics`
    Graphics2D,
    /// Application for viewing, creating, or processing vector graphics `Graphics;2DGraphics`
    VectorGraphics,
    /// Application for viewing, creating, or processing raster (bitmap) graphics `Graphics;2DGraphics`
    RasterGraphics,
    /// Application for viewing, creating, or processing 3-D graphics `Graphics`
    Graphics3D,
    /// Tool to scan a file/text `Graphics`
    Scanning,
    /// Optical character recognition application `Graphics;Scanning`
    OCR,
    /// Camera tools, etc. `Graphics` or `Office`
    Photography,
    /// Desktop Publishing applications and Color Management tools `Graphics` or `Office`
    Publishing,
    /// Tool to view e.g. a graphic or pdf file `Graphics` or `Office`
    Viewer,
    /// A text tool utility `Utility`
    TextTools,
    /// Configuration tool for the GUI `Settings`
    DesktopSettings,
    /// A tool to manage hardware components, like sound cards, video cards or printers `Settings`
    HardwareSettings,
    /// A tool to manage printers `HardwareSettings;Settings`
    Printing,
    /// A package manager application `Settings`
    PackageManager,
    /// A dial-up program `Network`
    Dialup,
    /// An instant messaging client `Network`
    InstantMessaging,
    /// A chat client `Network`
    Chat,
    /// An IRC client `Network`
    IRCClient,
    /// RSS, podcast and other subscription based contents `Network`
    Feed,
    /// Tools like FTP or P2P programs `Network`
    FileTransfer,
    /// HAM radio software `Network` or `Audio`
    HamRadio,
    /// A news reader or a news ticker `Network`
    News,
    /// A P2P program `Network`
    P2P,
    /// A tool to remotely manage your PC `Network`
    RemoteAccess,
    /// Telephony via PC `Network`
    Telephony,
    /// Telephony tools, to dial a number, manage PBX, ... `Utility`
    TelephonyTools,
    /// Video Conference software `Network`
    VideoConference,
    /// A web browser `Network`
    WebBrowser,
    /// A tool for web developers `Network` or `Development`
    WebDevelopment,
    /// An app related to MIDI `AudioVideo;Audio` 
    Midi,
    /// Just a mixer `AudioVideo;Audio`
    Mixer,
    /// A sequencer `AudioVideo;Audio`
    Sequencer,
    /// A tuner `AudioVideo;Audio`
    Tuner,
    /// A TV application `AudioVideo;Video`
    TV,
    /// Application to edit audio/video files `Audio` or `Video` or`Audio`
    AudioVideoEditing,
    /// Application to play audio/video files `Audio` or `Video` or`AudioVideo`
    VideoPlayer,
    /// Application to record audio/video files `Audio` or `Video` or `AudioVideo`
    Recorder,
    /// Application to burn a disc `AudioVideo`
    DiscBurning,
    /// An action game `Game`
    ActionGame,
    /// Adventure style game `Game`
    AdventureGame,
    /// Arcade style game `Game`
    ArcadeGame,
    /// A board game `Game`
    BoardGame,
    /// Falling blocks game `Game`
    BlocksGame,
    /// A card game `Game`
    CardGame,
    /// A game for kids `Game`
    KidsGame,
    /// Logic games like puzzles, etc `Game` 
    LogicGame,
    /// A role playing game `Game`
    RolePlaying,
    /// A shooter game `Game`
    Shooter,
    /// A simulation game `Game`
    Simulation,
    /// A sports game `Game`
    SportsGame,
    /// A strategy game `Game`
    StrategyGame,
    /// Software to teach arts `Education` or `Science`` 
    Art,
    ///``Education` or `Science``
    Construction,
    /// Musical software `AudioVideo` or `Education`
    Music,
    /// Software to learn foreign languages `Education` or `Science`
    Languages,
    /// Artificial Intelligence software `Education` or `Science`
    ArtificialIntelligence,
    /// Astronomy software `Education` or `Science`
    Astronomy,
    /// Biology software `Education` or `Science`
    Biology,
    /// Chemistry software `Education` or `Science`
    Chemistry,
    /// ComputerSience software `Education` or `Science`
    ComputerScience,
    /// Data visualization software `Education` or `Science`
    DataVisualization,
    /// Economy software `Education` or `Science`
    Economy,
    /// Electricity software `Education` or `Science`
    Electricity,
    /// Geography software `Education` or `Science`
    Geography,
    /// Geology software `Education` or `Science`
    Geology,
    /// Geoscience software, GIS `Education` or `Science`
    Geoscience,
    /// History software `Education` or `Science`
    History,
    /// Software for philosophy, psychology and other humanities `Education` or `Science`
    Humanities,
    /// Image Processing software `Education` or `Science`
    ImageProcessing,
    /// Literature software `Education` or `Science`
    Literature,
    /// Sofware for viewing maps, navigation, mapping, GPS `Education` or `Science` or `Utility` 
    Maps,
    /// Math software `Education` or `Science` 
    Math,
    /// Numerical analysis software `Education;Math` or `Science;Math` 
    NumericalAnalysis,
    /// Medical software `Education` or `Science` 
    MedicalSoftware,
    /// Physics software `Education` or `Science`
    Physics,
    /// Robotics software `Education` or `Science`
    Robotics,
    /// Religious and spiritual software, theology `Education` or `Science` or `Utility`
    Spirituality,
    /// Sports software `Education` or `Science`
    Sports,
    /// Parallel computing software `Education;ComputerScience` or `Science;ComputerScience`
    ParallelComputing,
    /// A simple amusement
    Amusement,
    /// A tool to archive/backup data `Utility`
    Archiving,
    /// A tool to manage compressed data/archives `Utility;Archiving`
    Compression,
    /// Electronics software, e.g. a circuit designer
    Electronics,
    /// Emulator of another platform, such as a DOS emulator `System` or `Game`
    Emulator,
    /// Engineering software, e.g. CAD programs
    Engineering,
    /// A file tool utility `Utility` or `System`
    FileTools,
    /// A file manager `System;FileTools`
    FileManager,
    /// A terminal emulator application `System`
    TerminalEmulator,
    /// A file system tool `System`
    Filesystem,
    /// Monitor application/applet that monitors some resource or activity `System` or `Network`
    Monitor,
    /// A security tool `Settings` or `System`
    Security,
    /// Accessibility `Settings` or `Utility`
    Accessibility,
    /// A calculator `Utility`
    Calculator,
    /// A clock application/applet `Utility`
    Clock,
    /// A text editor `Utility`
    TextEditor,
    /// Help or documentation
    Documentation,
    /// Application handles adult or explicit material
    Adult,
    /// Important application, core to the desktop such as a file manager or a help browser
    Core,
    /// Application based on KDE libraries `QT`
    KDE,
    /// Application based on GNOME libraries `GTK`
    GNOME,
    /// Application based on XFCE libraries `GTK`
    XFCE,
    /// Application based on GTK+ libraries
    GTK,
    /// Application based on Qt libraries
    Qt,
    /// Application based on Motif libraries
    Motif,
    /// Application based on Java GUI libraries, such as AWT or Swing
    Java,
    /// Application that only works inside a terminal (text-based or command line application)
    ConsoleOnly,
    /// This is for random people making whatever they want
    None,
}
impl fmt::Display for Categories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String = match *self {
            Categories::AudioVideo => "AudioVideo".to_string(),
            Categories::Audio => "Audio".to_string(),
            Categories::Video => "Video".to_string(),
            Categories::Education => "Education".to_string(),
            Categories::Game => "Game".to_string(),
            Categories::Graphics => "Graphics".to_string(),
            Categories::Network => "Network".to_string(),
            Categories::Office => "Office".to_string(),
            Categories::Science => "Science".to_string(),
            Categories::Settings => "Settings".to_string(),
            Categories::System => "System".to_string(),
            Categories::Utility => "Utility".to_string(),
// ADDITIONAL CATEGORIES ---------------------------------------------------------------------------------------------------------------------
            Categories::Building => "Building".to_string(),
            Categories::Debugger => "Debugger".to_string(),
            Categories::IDE => "IDE".to_string(),
            Categories::GUIDesigner => "GUIDesigner".to_string(),
            Categories::Profiling => "Profiling".to_string(),
            Categories::RevisionControl => "RevisionControl".to_string(),
            Categories::Translation => "Translation".to_string(),
            Categories::Calendar => "Calendar".to_string(),
            Categories::ContactManagement => "ContactManagement".to_string(),
            Categories::Database => "Database".to_string(),
            Categories::Dictionary => "Dictionary".to_string(),
            Categories::Chart => "Chart".to_string(),
            Categories::Email => "Email".to_string(),
            Categories::Finance => "Finance".to_string(),
            Categories::FlowChart => "FlowChart".to_string(),
            Categories::PDA => "PDA".to_string(),
            Categories::ProjectManagement => "ProjectManagement".to_string(),
            Categories::Presentation => "Presentation".to_string(),
            Categories::Spreadsheet => "Spreadsheet".to_string(),
            Categories::WordProcessor => "WordProcessor".to_string(),
            Categories::Graphics2D => "2DGraphics".to_string(),
            Categories::VectorGraphics => "VectorGraphics".to_string(),
            Categories::RasterGraphics => "RasterGraphics".to_string(),
            Categories::Graphics3D => "3DGraphics".to_string(),
            Categories::Scanning => "Scanning".to_string(),
            Categories::OCR => "OCR".to_string(),
            Categories::Photography => "Photography".to_string(),
            Categories::Publishing => "Publishing".to_string(),
            Categories::Viewer => "Viewer".to_string(),
            Categories::TextTools => "TextTools".to_string(),
            Categories::DesktopSettings => "DesktopSettings".to_string(),
            Categories::HardwareSettings => "HardwareSettings".to_string(),
            Categories::Printing => "Printing".to_string(),
            Categories::PackageManager => "PackageManager".to_string(),
            Categories::Dialup => "Dialup".to_string(),
            Categories::InstantMessaging => "InstantMessaging".to_string(),
            Categories::Chat => "Chat".to_string(),
            Categories::IRCClient => "IRCClient".to_string(),
            Categories::Feed => "Feed".to_string(),
            Categories::FileTransfer => "FileTransfer".to_string(),
            Categories::HamRadio => "HamRadio".to_string(),
            Categories::News => "News".to_string(),
            Categories::P2P => "P2P".to_string(),
            Categories::RemoteAccess => "RemoteAccess".to_string(),
            Categories::Telephony => "Telephony".to_string(),
            Categories::TelephonyTools => "TelephonyTools".to_string(),
            Categories::VideoConference => "VideoConference".to_string(),
            Categories::WebBrowser => "WebBrowser".to_string(),
            Categories::WebDevelopment => "WebDevelopment".to_string(),
            Categories::Midi => "Midi".to_string(),
            Categories::Mixer => "Mixer".to_string(),
            Categories::Sequencer => "Sequencer".to_string(),
            Categories::Tuner => "Tuner".to_string(),
            Categories::TV => "TV".to_string(),
            Categories::AudioVideoEditing => "AudioVideoEditing".to_string(),
            Categories::VideoPlayer => "VideoPlayer".to_string(),
            Categories::Recorder => "Recorder".to_string(),
            Categories::DiscBurning => "DiscBurning".to_string(),
            Categories::ActionGame => "ActionGame".to_string(),
            Categories::AdventureGame => "AdventureGame".to_string(),
            Categories::ArcadeGame => "ArcadeGame".to_string(),
            Categories::BoardGame => "BoardGame".to_string(),
            Categories::BlocksGame => "BlocksGame".to_string(),
            Categories::CardGame => "CardGame".to_string(),
            Categories::KidsGame => "KidsGame".to_string(),
            Categories::LogicGame => "LogicGame".to_string(),
            Categories::RolePlaying => "RolePlaying".to_string(),
            Categories::Shooter => "Shooter".to_string(),
            Categories::Simulation => "Simulation".to_string(),
            Categories::SportsGame => "SportsGame".to_string(),
            Categories::StrategyGame => "StrategyGame".to_string(),
            Categories::Art => "Art".to_string(),
            Categories::Construction => "Construction".to_string(),
            Categories::Music => "Music".to_string(),
            Categories::Languages => "Languages".to_string(),
            Categories::ArtificialIntelligence => "ArtificialIntelligence".to_string(),
            Categories::Astronomy => "Astronomy".to_string(),
            Categories::Biology => "Biology".to_string(),
            Categories::Chemistry => "Chemistry".to_string(),
            Categories::ComputerScience => "ComputerScience".to_string(),
            Categories::DataVisualization => "DataVisualization".to_string(),
            Categories::Economy => "Economy".to_string(),
            Categories::Electricity => "Electricity".to_string(),
            Categories::Geography => "Geography".to_string(),
            Categories::Geology => "Geology".to_string(),
            Categories::Geoscience => "Geoscience".to_string(),
            Categories::History => "History".to_string(),
            Categories::Humanities => "Humanities".to_string(),
            Categories::ImageProcessing => "ImageProcessing".to_string(),
            Categories::Literature => "Literature".to_string(),
            Categories::Maps => "Maps".to_string(),
            Categories::Math => "Math".to_string(),
            Categories::NumericalAnalysis => "NumericalAnalysis".to_string(),
            Categories::MedicalSoftware => "MedicalSoftware".to_string(),
            Categories::Physics => "Physics".to_string(),
            Categories::Robotics => "Robotics".to_string(),
            Categories::Spirituality => "Spirituality".to_string(),
            Categories::Sports => "Sports".to_string(),
            Categories::ParallelComputing => "ParallelComputing".to_string(),
            Categories::Amusement => "Amusement".to_string(),
            Categories::Archiving => "Archiving".to_string(),
            Categories::Compression => "Compression".to_string(),
            Categories::Electronics => "Electronics".to_string(),
            Categories::Emulator => "Emulator".to_string(),
            Categories::Engineering => "Engineering".to_string(),
            Categories::FileTools => "FileTools".to_string(),
            Categories::FileManager => "FileManager".to_string(),
            Categories::TerminalEmulator => "TerminalEmulator".to_string(),
            Categories::Filesystem => "Filesystem".to_string(),
            Categories::Monitor => "Monitor".to_string(),
            Categories::Security => "Security".to_string(),
            Categories::Accessibility => "Accessibility".to_string(),
            Categories::Calculator => "Calculator".to_string(),
            Categories::Clock => "Clock".to_string(),
            Categories::TextEditor => "TextEditor".to_string(),
            Categories::Documentation => "Documentation".to_string(),
            Categories::Adult => "Adult".to_string(),
            Categories::Core => "Core".to_string(),
            Categories::KDE => "KDE".to_string(),
            Categories::GNOME => "GNOME".to_string(),
            Categories::XFCE => "XFCE".to_string(),
            Categories::GTK => "GTK".to_string(),
            Categories::Qt => "Qt".to_string(),
            Categories::Motif => "Motif".to_string(),
            Categories::Java => "Java".to_string(),
            Categories::ConsoleOnly => "ConsoleOnly".to_string(),
            Categories::None => "None".to_string(),
        };
        write!(f, "{:?}", v)
    }
}
#[allow(dead_code, clippy::if_same_then_else)]

impl Categories {
    /// This function returns a `Categories` based on a matching string
    pub fn from_string(item:String) ->  Categories {
        if item == "AudioVideo" {
            Categories::AudioVideo
        } else if item == "Audio" {
            Categories::Audio
        } else if item == "Video" {
            Categories::Video
        } else if item == "Education" {
            Categories::Education
        } else if item == "Game" {
            Categories::Game
        } else if item == "Graphics" {
            Categories::Graphics
        } else if item == "Network" {
            Categories::Network
        } else if item == "Office" {
            Categories::Office
        } else if item == "Science" {
            Categories::Science
        } else if item == "Settings" {
            Categories::Settings
        } else if item == "System" {
            Categories::System
        } else if item == "Utility" {
            Categories::Utility
// ADDITIONAL CATEGORIES ---------------------------------------------------------------------------------------------------------------------

        } else if item == "Building" {
            Categories::Building
        } else if item == "Debugger" {
            Categories::Debugger
        } else if item == "IDE" {
            Categories::IDE
        } else if item == "GUIDesigner" {
            Categories::GUIDesigner
        } else if item == "Profiling" {
            Categories::Profiling
        } else if item == "RevisionControl" {
            Categories::RevisionControl
        } else if item == "Translation" {
            Categories::Translation
        } else if item == "Calendar" {
            Categories::Calendar
        } else if item == "ContactManagement" {
            Categories::ContactManagement
        } else if item == "Database" {
            Categories::Database
        } else if item == "Dictionary" {
            Categories::Dictionary
        } else if item == "Chart" {
            Categories::Chart
        } else if item == "Email" {
            Categories::Email
        } else if item == "Finance" {
            Categories::Finance
        } else if item == "FlowChart" {
            Categories::FlowChart
        } else if item == "PDA" {
            Categories::PDA
        } else if item == "ProjectManagement" {
            Categories::ProjectManagement
        } else if item == "Presentation" {
            Categories::Presentation
        } else if item == "Spreadsheet" {
            Categories::Spreadsheet
        } else if item == "WordProcessor" {
            Categories::WordProcessor
        } else if item == "Graphics2D" {
            Categories::Graphics2D
        } else if item == "2DGraphics" {
            Categories::Graphics2D
        } else if item == "VectorGraphics" {
            Categories::VectorGraphics
        } else if item == "RasterGraphics" {
            Categories::RasterGraphics
        } else if item == "Graphics3D" {
            Categories::Graphics3D
        } else if item == "3DGraphics" {
            Categories::Graphics3D
        } else if item == "Scanning" {
            Categories::Scanning
        } else if item == "OCR" {
            Categories::OCR
        } else if item == "Photography" {
            Categories::Photography
        } else if item == "Publishing" {
            Categories::Publishing
        } else if item == "Viewer" {
            Categories::Viewer
        } else if item == "TextTools" {
            Categories::TextTools
        } else if item == "DesktopSettings" {
            Categories::DesktopSettings
        } else if item == "HardwareSettings" {
            Categories::HardwareSettings
        } else if item == "Printing" {
            Categories::Printing
        } else if item == "PackageManager" {
            Categories::PackageManager
        } else if item == "Dialup" {
            Categories::Dialup
        } else if item == "InstantMessaging" {
            Categories::InstantMessaging
        } else if item == "Chat" {
            Categories::Chat
        } else if item == "IRCClient" {
            Categories::IRCClient
        } else if item == "Feed" {
            Categories::Feed
        } else if item == "FileTransfer" {
            Categories::FileTransfer
        } else if item == "HamRadio" {
            Categories::HamRadio
        } else if item == "News" {
            Categories::News
        } else if item == "P2P" {
            Categories::P2P
        } else if item == "RemoteAccess" {
            Categories::RemoteAccess
        } else if item == "Telephony" {
            Categories::Telephony
        } else if item == "TelephonyTools" {
            Categories::TelephonyTools
        } else if item == "VideoConference" {
            Categories::VideoConference
        } else if item == "WebBrowser" {
            Categories::WebBrowser
        } else if item == "WebDevelopment" {
            Categories::WebDevelopment
        } else if item == "Midi" {
            Categories::Midi
        } else if item == "Mixer" {
            Categories::Mixer
        } else if item == "Sequencer" {
            Categories::Sequencer
        } else if item == "Tuner" {
            Categories::Tuner
        } else if item == "TV" {
            Categories::TV
        } else if item == "AudioVideoEditing" {
            Categories::AudioVideoEditing
        } else if item == "VideoPlayer" {
            Categories::VideoPlayer
        } else if item == "Recorder" {
            Categories::Recorder
        } else if item == "DiscBurning" {
            Categories::DiscBurning
        } else if item == "ActionGame" {
            Categories::ActionGame
        } else if item == "AdventureGame" {
            Categories::AdventureGame
        } else if item == "ArcadeGame" {
            Categories::ArcadeGame
        } else if item == "BoardGame" {
            Categories::BoardGame
        } else if item == "BlocksGame" {
            Categories::BlocksGame
        } else if item == "CardGame" {
            Categories::CardGame
        } else if item == "KidsGame" {
            Categories::KidsGame
        } else if item == "LogicGame" {
            Categories::LogicGame
        } else if item == "RolePlaying" {
            Categories::RolePlaying
        } else if item == "Shooter" {
            Categories::Shooter
        } else if item == "Simulation" {
            Categories::Simulation
        } else if item == "SportsGame" {
            Categories::SportsGame
        } else if item == "StrategyGame" {
            Categories::StrategyGame
        } else if item == "Art" {
            Categories::Art
        } else if item == "Construction" {
            Categories::Construction
        } else if item == "Music" {
            Categories::Music
        } else if item == "Languages" {
            Categories::Languages
        } else if item == "ArtificialIntelligence" {
            Categories::ArtificialIntelligence
        } else if item == "Astronomy" {
            Categories::Astronomy
        } else if item == "Biology" {
            Categories::Biology
        } else if item == "Chemistry" {
            Categories::Chemistry
        } else if item == "ComputerScience" {
            Categories::ComputerScience
        } else if item == "DataVisualization" {
            Categories::DataVisualization
        } else if item == "Economy" {
            Categories::Economy
        } else if item == "Electricity" {
            Categories::Electricity
        } else if item == "Geography" {
            Categories::Geography
        } else if item == "Geology" {
            Categories::Geology
        } else if item == "Geoscience" {
            Categories::Geoscience
        } else if item == "History" {
            Categories::History
        } else if item == "Humanities" {
            Categories::Humanities
        } else if item == "ImageProcessing" {
            Categories::ImageProcessing
        } else if item == "Literature" {
            Categories::Literature
        } else if item == "Maps" {
            Categories::Maps
        } else if item == "Math" {
            Categories::Math
        } else if item == "NumericalAnalysis" {
            Categories::NumericalAnalysis
        } else if item == "MedicalSoftware" {
            Categories::MedicalSoftware
        } else if item == "Physics" {
            Categories::Physics
        } else if item == "Robotics" {
            Categories::Robotics
        } else if item == "Spirituality" {
            Categories::Spirituality
        } else if item == "Sports" {
            Categories::Sports
        } else if item == "ParallelComputing" {
            Categories::ParallelComputing
        } else if item == "Amusement" {
            Categories::Amusement
        } else if item == "Archiving" {
            Categories::Archiving
        } else if item == "Compression" {
            Categories::Compression
        } else if item == "Electronics" {
            Categories::Electronics
        } else if item == "Emulator" {
            Categories::Emulator
        } else if item == "Engineering" {
            Categories::Engineering
        } else if item == "FileTools" {
            Categories::FileTools
        } else if item == "FileManager" {
            Categories::FileManager
        } else if item == "TerminalEmulator" {
            Categories::TerminalEmulator
        } else if item == "Filesystem" {
            Categories::Filesystem
        } else if item == "Monitor" {
            Categories::Monitor
        } else if item == "Security" {
            Categories::Security
        } else if item == "Accessibility" {
            Categories::Accessibility
        } else if item == "Calculator" {
            Categories::Calculator
        } else if item == "Clock" {
            Categories::Clock
        } else if item == "TextEditor" {
            Categories::TextEditor
        } else if item == "Documentation" {
            Categories::Documentation
        } else if item == "Adult" {
            Categories::Adult
        } else if item == "Core" {
            Categories::Core
        } else if item == "KDE" {
            Categories::KDE
        } else if item == "GNOME" {
            Categories::GNOME
        } else if item == "XFCE" {
            Categories::XFCE
        } else if item == "GTK" {
            Categories::GTK
        } else if item == "Qt" {
            Categories::Qt
        } else if item == "Motif" {
            Categories::Motif
        } else if item == "Java" {
            Categories::Java
        } else if item == "ConsoleOnly" {
            Categories::ConsoleOnly
        } else  {
            Categories::None
        }
    }
}
impl Default for Categories {
    fn default() -> Self {
        Self::None
    }
}
