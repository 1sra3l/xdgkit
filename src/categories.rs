/*!
# Categories

The rust enum-ification of the XDG Desktop Entry Categories specifications.

Eventually I aim to implement converting all the "additional" categories to/from `String`
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
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
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
    /// This is a stub to include all the Additional Categories
    Other(AdditionalCategories),
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
            Categories::Other(_t) => "Other".to_string(),
        };
        write!(f, "{:?}", v)
    }
}
#[allow(dead_code)]
impl Categories {
    /// This function returns a `Categories` based on a matching string
    pub fn from_string(item:String) ->  Categories{
        if item == "AudioVideo" {
            return Categories::AudioVideo
        } else if item == "Audio" {
            return Categories::Audio
        } else if item == "Video" {
            return Categories::Video
        } else if item == "Education" {
            return Categories::Education
        } else if item == "Game" {
            return Categories::Game
        } else if item == "Graphics" {
            return Categories::Graphics
        } else if item == "Network" {
            return Categories::Network
        } else if item == "Office" {
            return Categories::Office
        } else if item == "Science" {
            return Categories::Science
        } else if item == "Settings" {
            return Categories::Settings
        } else if item == "System" {
            return Categories::System
        } else if item == "Utility" {
            return Categories::Utility        
        }
        Categories::Other(get_additional(item))

    }
}
//TODO
/// This function is intended to go through the "AdditionalCategories" below and string-ify them
pub fn get_additional(item:String) -> AdditionalCategories{
    if item == "Unknown" {
        return AdditionalCategories::Unknown
    }
    AdditionalCategories::Unknown
}

/// The Additional Categories
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum AdditionalCategories {
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
    /// This is for random people making whatever they want... `Unknown` is similar to Desktop Entry's `type`
    Unknown,
}
impl AdditionalCategories {
    pub fn from_string(item:String) ->  AdditionalCategories {
        if item == "Building" {
            return AdditionalCategories::Building
        } else if item == "Debugger" {
            return AdditionalCategories::Debugger
        } else if item == "IDE" {
            return AdditionalCategories::IDE
        } else if item == "GUIDesigner" {
            return AdditionalCategories::GUIDesigner
        } else if item == "Profiling" {
            return AdditionalCategories::Profiling
        } else if item == "RevisionControl" {
            return AdditionalCategories::RevisionControl
        } else if item == "Translation" {
            return AdditionalCategories::Translation
        } else if item == "Calendar" {
            return AdditionalCategories::Calendar
        } else if item == "ContactManagement" {
            return AdditionalCategories::ContactManagement
        } else if item == "Database" {
            return AdditionalCategories::Database
        } else if item == "Dictionary" {
            return AdditionalCategories::Dictionary
        } else if item == "Chart" {
            return AdditionalCategories::Chart
        } else if item == "Email" {
            return AdditionalCategories::Email
        } else if item == "Finance" {
            return AdditionalCategories::Finance
        } else if item == "FlowChart" {
            return AdditionalCategories::FlowChart
        } else if item == "PDA" {
            return AdditionalCategories::PDA
        } else if item == "ProjectManagement" {
            return AdditionalCategories::ProjectManagement
        } else if item == "Presentation" {
            return AdditionalCategories::Presentation
        } else if item == "Spreadsheet" {
            return AdditionalCategories::Spreadsheet
        } else if item == "WordProcessor" {
            return AdditionalCategories::WordProcessor
        } else if item == "Graphics2D" {
            return AdditionalCategories::Graphics2D
        } else if item == "2DGraphics" {
            return AdditionalCategories::Graphics2D
        } else if item == "VectorGraphics" {
            return AdditionalCategories::VectorGraphics
        } else if item == "RasterGraphics" {
            return AdditionalCategories::RasterGraphics
        } else if item == "Graphics3D" {
            return AdditionalCategories::Graphics3D
        } else if item == "3DGraphics" {
            return AdditionalCategories::Graphics3D
        } else if item == "Scanning" {
            return AdditionalCategories::Scanning
        } else if item == "OCR" {
            return AdditionalCategories::OCR
        } else if item == "Photography" {
            return AdditionalCategories::Photography
        } else if item == "Publishing" {
            return AdditionalCategories::Publishing
        } else if item == "Viewer" {
            return AdditionalCategories::Viewer
        } else if item == "TextTools" {
            return AdditionalCategories::TextTools
        } else if item == "DesktopSettings" {
            return AdditionalCategories::DesktopSettings
        } else if item == "HardwareSettings" {
            return AdditionalCategories::HardwareSettings
        } else if item == "Printing" {
            return AdditionalCategories::Printing
        } else if item == "PackageManager" {
            return AdditionalCategories::PackageManager
        } else if item == "Dialup" {
            return AdditionalCategories::Dialup
        } else if item == "InstantMessaging" {
            return AdditionalCategories::InstantMessaging
        } else if item == "Chat" {
            return AdditionalCategories::Chat
        } else if item == "IRCClient" {
            return AdditionalCategories::IRCClient
        } else if item == "Feed" {
            return AdditionalCategories::Feed
        } else if item == "FileTransfer" {
            return AdditionalCategories::FileTransfer
        } else if item == "HamRadio" {
            return AdditionalCategories::HamRadio
        } else if item == "News" {
            return AdditionalCategories::News
        } else if item == "P2P" {
            return AdditionalCategories::P2P
        } else if item == "RemoteAccess" {
            return AdditionalCategories::RemoteAccess
        } else if item == "Telephony" {
            return AdditionalCategories::Telephony
        } else if item == "TelephonyTools" {
            return AdditionalCategories::TelephonyTools
        } else if item == "VideoConference" {
            return AdditionalCategories::VideoConference
        } else if item == "WebBrowser" {
            return AdditionalCategories::WebBrowser
        } else if item == "WebDevelopment" {
            return AdditionalCategories::WebDevelopment
        } else if item == "Midi" {
            return AdditionalCategories::Midi
        } else if item == "Mixer" {
            return AdditionalCategories::Mixer
        } else if item == "Sequencer" {
            return AdditionalCategories::Sequencer
        } else if item == "Tuner" {
            return AdditionalCategories::Tuner
        } else if item == "TV" {
            return AdditionalCategories::TV
        } else if item == "AudioVideoEditing" {
            return AdditionalCategories::AudioVideoEditing
        } else if item == "VideoPlayer" {
            return AdditionalCategories::VideoPlayer
        } else if item == "Recorder" {
            return AdditionalCategories::Recorder
        } else if item == "DiscBurning" {
            return AdditionalCategories::DiscBurning
        } else if item == "ActionGame" {
            return AdditionalCategories::ActionGame
        } else if item == "AdventureGame" {
            return AdditionalCategories::AdventureGame
        } else if item == "ArcadeGame" {
            return AdditionalCategories::ArcadeGame
        } else if item == "BoardGame" {
            return AdditionalCategories::BoardGame
        } else if item == "BlocksGame" {
            return AdditionalCategories::BlocksGame
        } else if item == "CardGame" {
            return AdditionalCategories::CardGame
        } else if item == "KidsGame" {
            return AdditionalCategories::KidsGame
        } else if item == "LogicGame" {
            return AdditionalCategories::LogicGame
        } else if item == "RolePlaying" {
            return AdditionalCategories::RolePlaying
        } else if item == "Shooter" {
            return AdditionalCategories::Shooter
        } else if item == "Simulation" {
            return AdditionalCategories::Simulation
        } else if item == "SportsGame" {
            return AdditionalCategories::SportsGame
        } else if item == "StrategyGame" {
            return AdditionalCategories::StrategyGame
        } else if item == "Art" {
            return AdditionalCategories::Art
        } else if item == "Construction" {
            return AdditionalCategories::Construction
        } else if item == "Music" {
            return AdditionalCategories::Music
        } else if item == "Languages" {
            return AdditionalCategories::Languages
        } else if item == "ArtificialIntelligence" {
            return AdditionalCategories::ArtificialIntelligence
        } else if item == "Astronomy" {
            return AdditionalCategories::Astronomy
        } else if item == "Biology" {
            return AdditionalCategories::Biology
        } else if item == "Chemistry" {
            return AdditionalCategories::Chemistry
        } else if item == "ComputerScience" {
            return AdditionalCategories::ComputerScience
        } else if item == "DataVisualization" {
            return AdditionalCategories::DataVisualization
        } else if item == "Economy" {
            return AdditionalCategories::Economy
        } else if item == "Electricity" {
            return AdditionalCategories::Electricity
        } else if item == "Geography" {
            return AdditionalCategories::Geography
        } else if item == "Geology" {
            return AdditionalCategories::Geology
        } else if item == "Geoscience" {
            return AdditionalCategories::Geoscience
        } else if item == "History" {
            return AdditionalCategories::History
        } else if item == "Humanities" {
            return AdditionalCategories::Humanities
        } else if item == "ImageProcessing" {
            return AdditionalCategories::ImageProcessing
        } else if item == "Literature" {
            return AdditionalCategories::Literature
        } else if item == "Maps" {
            return AdditionalCategories::Maps
        } else if item == "Math" {
            return AdditionalCategories::Math
        } else if item == "NumericalAnalysis" {
            return AdditionalCategories::NumericalAnalysis
        } else if item == "MedicalSoftware" {
            return AdditionalCategories::MedicalSoftware
        } else if item == "Physics" {
            return AdditionalCategories::Physics
        } else if item == "Robotics" {
            return AdditionalCategories::Robotics
        } else if item == "Spirituality" {
            return AdditionalCategories::Spirituality
        } else if item == "Sports" {
            return AdditionalCategories::Sports
        } else if item == "ParallelComputing" {
            return AdditionalCategories::ParallelComputing
        } else if item == "Amusement" {
            return AdditionalCategories::Amusement
        } else if item == "Archiving" {
            return AdditionalCategories::Archiving
        } else if item == "Compression" {
            return AdditionalCategories::Compression
        } else if item == "Electronics" {
            return AdditionalCategories::Electronics
        } else if item == "Emulator" {
            return AdditionalCategories::Emulator
        } else if item == "Engineering" {
            return AdditionalCategories::Engineering
        } else if item == "FileTools" {
            return AdditionalCategories::FileTools
        } else if item == "FileManager" {
            return AdditionalCategories::FileManager
        } else if item == "TerminalEmulator" {
            return AdditionalCategories::TerminalEmulator
        } else if item == "Filesystem" {
            return AdditionalCategories::Filesystem
        } else if item == "Monitor" {
            return AdditionalCategories::Monitor
        } else if item == "Security" {
            return AdditionalCategories::Security
        } else if item == "Accessibility" {
            return AdditionalCategories::Accessibility
        } else if item == "Calculator" {
            return AdditionalCategories::Calculator
        } else if item == "Clock" {
            return AdditionalCategories::Clock
        } else if item == "TextEditor" {
            return AdditionalCategories::TextEditor
        } else if item == "Documentation" {
            return AdditionalCategories::Documentation
        } else if item == "Adult" {
            return AdditionalCategories::Adult
        } else if item == "Core" {
            return AdditionalCategories::Core
        } else if item == "KDE" {
            return AdditionalCategories::KDE
        } else if item == "GNOME" {
            return AdditionalCategories::GNOME
        } else if item == "XFCE" {
            return AdditionalCategories::XFCE
        } else if item == "GTK" {
            return AdditionalCategories::GTK
        } else if item == "Qt" {
            return AdditionalCategories::Qt
        } else if item == "Motif" {
            return AdditionalCategories::Motif
        } else if item == "Java" {
            return AdditionalCategories::Java
        } else if item == "ConsoleOnly" {
            return AdditionalCategories::ConsoleOnly
        } else  {
            return AdditionalCategories::Unknown
        }
        AdditionalCategories::Unknown
    }
}
impl fmt::Display for AdditionalCategories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String = match *self {
            AdditionalCategories::Building => "Building".to_string(),
            AdditionalCategories::Debugger => "Debugger".to_string(),
            AdditionalCategories::IDE => "IDE".to_string(),
            AdditionalCategories::GUIDesigner => "GUIDesigner".to_string(),
            AdditionalCategories::Profiling => "Profiling".to_string(),
            AdditionalCategories::RevisionControl => "RevisionControl".to_string(),
            AdditionalCategories::Translation => "Translation".to_string(),
            AdditionalCategories::Calendar => "Calendar".to_string(),
            AdditionalCategories::ContactManagement => "ContactManagement".to_string(),
            AdditionalCategories::Database => "Database".to_string(),
            AdditionalCategories::Dictionary => "Dictionary".to_string(),
            AdditionalCategories::Chart => "Chart".to_string(),
            AdditionalCategories::Email => "Email".to_string(),
            AdditionalCategories::Finance => "Finance".to_string(),
            AdditionalCategories::FlowChart => "FlowChart".to_string(),
            AdditionalCategories::PDA => "PDA".to_string(),
            AdditionalCategories::ProjectManagement => "ProjectManagement".to_string(),
            AdditionalCategories::Presentation => "Presentation".to_string(),
            AdditionalCategories::Spreadsheet => "Spreadsheet".to_string(),
            AdditionalCategories::WordProcessor => "WordProcessor".to_string(),
            AdditionalCategories::Graphics2D => "2DGraphics".to_string(),
            AdditionalCategories::VectorGraphics => "VectorGraphics".to_string(),
            AdditionalCategories::RasterGraphics => "RasterGraphics".to_string(),
            AdditionalCategories::Graphics3D => "3DGraphics".to_string(),
            AdditionalCategories::Scanning => "Scanning".to_string(),
            AdditionalCategories::OCR => "OCR".to_string(),
            AdditionalCategories::Photography => "Photography".to_string(),
            AdditionalCategories::Publishing => "Publishing".to_string(),
            AdditionalCategories::Viewer => "Viewer".to_string(),
            AdditionalCategories::TextTools => "TextTools".to_string(),
            AdditionalCategories::DesktopSettings => "DesktopSettings".to_string(),
            AdditionalCategories::HardwareSettings => "HardwareSettings".to_string(),
            AdditionalCategories::Printing => "Printing".to_string(),
            AdditionalCategories::PackageManager => "PackageManager".to_string(),
            AdditionalCategories::Dialup => "Dialup".to_string(),
            AdditionalCategories::InstantMessaging => "InstantMessaging".to_string(),
            AdditionalCategories::Chat => "Chat".to_string(),
            AdditionalCategories::IRCClient => "IRCClient".to_string(),
            AdditionalCategories::Feed => "Feed".to_string(),
            AdditionalCategories::FileTransfer => "FileTransfer".to_string(),
            AdditionalCategories::HamRadio => "HamRadio".to_string(),
            AdditionalCategories::News => "News".to_string(),
            AdditionalCategories::P2P => "P2P".to_string(),
            AdditionalCategories::RemoteAccess => "RemoteAccess".to_string(),
            AdditionalCategories::Telephony => "Telephony".to_string(),
            AdditionalCategories::TelephonyTools => "TelephonyTools".to_string(),
            AdditionalCategories::VideoConference => "VideoConference".to_string(),
            AdditionalCategories::WebBrowser => "WebBrowser".to_string(),
            AdditionalCategories::WebDevelopment => "WebDevelopment".to_string(),
            AdditionalCategories::Midi => "Midi".to_string(),
            AdditionalCategories::Mixer => "Mixer".to_string(),
            AdditionalCategories::Sequencer => "Sequencer".to_string(),
            AdditionalCategories::Tuner => "Tuner".to_string(),
            AdditionalCategories::TV => "TV".to_string(),
            AdditionalCategories::AudioVideoEditing => "AudioVideoEditing".to_string(),
            AdditionalCategories::VideoPlayer => "VideoPlayer".to_string(),
            AdditionalCategories::Recorder => "Recorder".to_string(),
            AdditionalCategories::DiscBurning => "DiscBurning".to_string(),
            AdditionalCategories::ActionGame => "ActionGame".to_string(),
            AdditionalCategories::AdventureGame => "AdventureGame".to_string(),
            AdditionalCategories::ArcadeGame => "ArcadeGame".to_string(),
            AdditionalCategories::BoardGame => "BoardGame".to_string(),
            AdditionalCategories::BlocksGame => "BlocksGame".to_string(),
            AdditionalCategories::CardGame => "CardGame".to_string(),
            AdditionalCategories::KidsGame => "KidsGame".to_string(),
            AdditionalCategories::LogicGame => "LogicGame".to_string(),
            AdditionalCategories::RolePlaying => "RolePlaying".to_string(),
            AdditionalCategories::Shooter => "Shooter".to_string(),
            AdditionalCategories::Simulation => "Simulation".to_string(),
            AdditionalCategories::SportsGame => "SportsGame".to_string(),
            AdditionalCategories::StrategyGame => "StrategyGame".to_string(),
            AdditionalCategories::Art => "Art".to_string(),
            AdditionalCategories::Construction => "Construction".to_string(),
            AdditionalCategories::Music => "Music".to_string(),
            AdditionalCategories::Languages => "Languages".to_string(),
            AdditionalCategories::ArtificialIntelligence => "ArtificialIntelligence".to_string(),
            AdditionalCategories::Astronomy => "Astronomy".to_string(),
            AdditionalCategories::Biology => "Biology".to_string(),
            AdditionalCategories::Chemistry => "Chemistry".to_string(),
            AdditionalCategories::ComputerScience => "ComputerScience".to_string(),
            AdditionalCategories::DataVisualization => "DataVisualization".to_string(),
            AdditionalCategories::Economy => "Economy".to_string(),
            AdditionalCategories::Electricity => "Electricity".to_string(),
            AdditionalCategories::Geography => "Geography".to_string(),
            AdditionalCategories::Geology => "Geology".to_string(),
            AdditionalCategories::Geoscience => "Geoscience".to_string(),
            AdditionalCategories::History => "History".to_string(),
            AdditionalCategories::Humanities => "Humanities".to_string(),
            AdditionalCategories::ImageProcessing => "ImageProcessing".to_string(),
            AdditionalCategories::Literature => "Literature".to_string(),
            AdditionalCategories::Maps => "Maps".to_string(),
            AdditionalCategories::Math => "Math".to_string(),
            AdditionalCategories::NumericalAnalysis => "NumericalAnalysis".to_string(),
            AdditionalCategories::MedicalSoftware => "MedicalSoftware".to_string(),
            AdditionalCategories::Physics => "Physics".to_string(),
            AdditionalCategories::Robotics => "Robotics".to_string(),
            AdditionalCategories::Spirituality => "Spirituality".to_string(),
            AdditionalCategories::Sports => "Sports".to_string(),
            AdditionalCategories::ParallelComputing => "ParallelComputing".to_string(),
            AdditionalCategories::Amusement => "Amusement".to_string(),
            AdditionalCategories::Archiving => "Archiving".to_string(),
            AdditionalCategories::Compression => "Compression".to_string(),
            AdditionalCategories::Electronics => "Electronics".to_string(),
            AdditionalCategories::Emulator => "Emulator".to_string(),
            AdditionalCategories::Engineering => "Engineering".to_string(),
            AdditionalCategories::FileTools => "FileTools".to_string(),
            AdditionalCategories::FileManager => "FileManager".to_string(),
            AdditionalCategories::TerminalEmulator => "TerminalEmulator".to_string(),
            AdditionalCategories::Filesystem => "Filesystem".to_string(),
            AdditionalCategories::Monitor => "Monitor".to_string(),
            AdditionalCategories::Security => "Security".to_string(),
            AdditionalCategories::Accessibility => "Accessibility".to_string(),
            AdditionalCategories::Calculator => "Calculator".to_string(),
            AdditionalCategories::Clock => "Clock".to_string(),
            AdditionalCategories::TextEditor => "TextEditor".to_string(),
            AdditionalCategories::Documentation => "Documentation".to_string(),
            AdditionalCategories::Adult => "Adult".to_string(),
            AdditionalCategories::Core => "Core".to_string(),
            AdditionalCategories::KDE => "KDE".to_string(),
            AdditionalCategories::GNOME => "GNOME".to_string(),
            AdditionalCategories::XFCE => "XFCE".to_string(),
            AdditionalCategories::GTK => "GTK".to_string(),
            AdditionalCategories::Qt => "Qt".to_string(),
            AdditionalCategories::Motif => "Motif".to_string(),
            AdditionalCategories::Java => "Java".to_string(),
            AdditionalCategories::ConsoleOnly => "ConsoleOnly".to_string(),
            AdditionalCategories::Unknown => "Unknown".to_string(),
        };
        write!(f, "{:?}", v)
    }
}

