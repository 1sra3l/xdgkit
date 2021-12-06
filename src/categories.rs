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
    AdditionalCategories::from_string(item)
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
    Ide,
    /// A GUI designer application `Development`
    GuiDesigner,
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
    Pda,
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
    Ocr,
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
    Tv,
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
    Kde,
    /// Application based on GNOME libraries `GTK`
    Gnome,
    /// Application based on XFCE libraries `GTK`
    Xfce,
    /// Application based on GTK+ libraries
    Gtk,
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
    #[allow(clippy::if_same_then_else)]
    pub fn from_string(item:String) ->  AdditionalCategories {
        if item == "Building" {
            AdditionalCategories::Building
        } else if item == "Debugger" {
            AdditionalCategories::Debugger
        } else if item == "IDE" {
            AdditionalCategories::Ide
        } else if item == "GUIDesigner" {
            AdditionalCategories::GuiDesigner
        } else if item == "Profiling" {
            AdditionalCategories::Profiling
        } else if item == "RevisionControl" {
            AdditionalCategories::RevisionControl
        } else if item == "Translation" {
            AdditionalCategories::Translation
        } else if item == "Calendar" {
            AdditionalCategories::Calendar
        } else if item == "ContactManagement" {
            AdditionalCategories::ContactManagement
        } else if item == "Database" {
            AdditionalCategories::Database
        } else if item == "Dictionary" {
            AdditionalCategories::Dictionary
        } else if item == "Chart" {
            AdditionalCategories::Chart
        } else if item == "Email" {
            AdditionalCategories::Email
        } else if item == "Finance" {
            AdditionalCategories::Finance
        } else if item == "FlowChart" {
            AdditionalCategories::FlowChart
        } else if item == "PDA" {
            AdditionalCategories::Pda
        } else if item == "ProjectManagement" {
            AdditionalCategories::ProjectManagement
        } else if item == "Presentation" {
            AdditionalCategories::Presentation
        } else if item == "Spreadsheet" {
            AdditionalCategories::Spreadsheet
        } else if item == "WordProcessor" {
            AdditionalCategories::WordProcessor
        } else if item == "Graphics2D" {
            AdditionalCategories::Graphics2D
        } else if item == "2DGraphics" {
            AdditionalCategories::Graphics2D
        } else if item == "VectorGraphics" {
            AdditionalCategories::VectorGraphics
        } else if item == "RasterGraphics" {
            AdditionalCategories::RasterGraphics
        } else if item == "Graphics3D" {
            AdditionalCategories::Graphics3D
        } else if item == "3DGraphics" {
            AdditionalCategories::Graphics3D
        } else if item == "Scanning" {
            AdditionalCategories::Scanning
        } else if item == "OCR" {
            AdditionalCategories::Ocr
        } else if item == "Photography" {
            AdditionalCategories::Photography
        } else if item == "Publishing" {
            AdditionalCategories::Publishing
        } else if item == "Viewer" {
            AdditionalCategories::Viewer
        } else if item == "TextTools" {
            AdditionalCategories::TextTools
        } else if item == "DesktopSettings" {
            AdditionalCategories::DesktopSettings
        } else if item == "HardwareSettings" {
            AdditionalCategories::HardwareSettings
        } else if item == "Printing" {
            AdditionalCategories::Printing
        } else if item == "PackageManager" {
            AdditionalCategories::PackageManager
        } else if item == "Dialup" {
            AdditionalCategories::Dialup
        } else if item == "InstantMessaging" {
            AdditionalCategories::InstantMessaging
        } else if item == "Chat" {
            AdditionalCategories::Chat
        } else if item == "IRCClient" {
            AdditionalCategories::IRCClient
        } else if item == "Feed" {
            AdditionalCategories::Feed
        } else if item == "FileTransfer" {
            AdditionalCategories::FileTransfer
        } else if item == "HamRadio" {
            AdditionalCategories::HamRadio
        } else if item == "News" {
            AdditionalCategories::News
        } else if item == "P2P" {
            AdditionalCategories::P2P
        } else if item == "RemoteAccess" {
            AdditionalCategories::RemoteAccess
        } else if item == "Telephony" {
            AdditionalCategories::Telephony
        } else if item == "TelephonyTools" {
            AdditionalCategories::TelephonyTools
        } else if item == "VideoConference" {
            AdditionalCategories::VideoConference
        } else if item == "WebBrowser" {
            AdditionalCategories::WebBrowser
        } else if item == "WebDevelopment" {
            AdditionalCategories::WebDevelopment
        } else if item == "Midi" {
            AdditionalCategories::Midi
        } else if item == "Mixer" {
            AdditionalCategories::Mixer
        } else if item == "Sequencer" {
            AdditionalCategories::Sequencer
        } else if item == "Tuner" {
            AdditionalCategories::Tuner
        } else if item == "TV" {
            AdditionalCategories::Tv
        } else if item == "AudioVideoEditing" {
            AdditionalCategories::AudioVideoEditing
        } else if item == "VideoPlayer" {
            AdditionalCategories::VideoPlayer
        } else if item == "Recorder" {
            AdditionalCategories::Recorder
        } else if item == "DiscBurning" {
            AdditionalCategories::DiscBurning
        } else if item == "ActionGame" {
            AdditionalCategories::ActionGame
        } else if item == "AdventureGame" {
            AdditionalCategories::AdventureGame
        } else if item == "ArcadeGame" {
            AdditionalCategories::ArcadeGame
        } else if item == "BoardGame" {
            AdditionalCategories::BoardGame
        } else if item == "BlocksGame" {
            AdditionalCategories::BlocksGame
        } else if item == "CardGame" {
            AdditionalCategories::CardGame
        } else if item == "KidsGame" {
            AdditionalCategories::KidsGame
        } else if item == "LogicGame" {
            AdditionalCategories::LogicGame
        } else if item == "RolePlaying" {
            AdditionalCategories::RolePlaying
        } else if item == "Shooter" {
            AdditionalCategories::Shooter
        } else if item == "Simulation" {
            AdditionalCategories::Simulation
        } else if item == "SportsGame" {
            AdditionalCategories::SportsGame
        } else if item == "StrategyGame" {
            AdditionalCategories::StrategyGame
        } else if item == "Art" {
            AdditionalCategories::Art
        } else if item == "Construction" {
            AdditionalCategories::Construction
        } else if item == "Music" {
            AdditionalCategories::Music
        } else if item == "Languages" {
            AdditionalCategories::Languages
        } else if item == "ArtificialIntelligence" {
            AdditionalCategories::ArtificialIntelligence
        } else if item == "Astronomy" {
            AdditionalCategories::Astronomy
        } else if item == "Biology" {
            AdditionalCategories::Biology
        } else if item == "Chemistry" {
            AdditionalCategories::Chemistry
        } else if item == "ComputerScience" {
            AdditionalCategories::ComputerScience
        } else if item == "DataVisualization" {
            AdditionalCategories::DataVisualization
        } else if item == "Economy" {
            AdditionalCategories::Economy
        } else if item == "Electricity" {
            AdditionalCategories::Electricity
        } else if item == "Geography" {
            AdditionalCategories::Geography
        } else if item == "Geology" {
            AdditionalCategories::Geology
        } else if item == "Geoscience" {
            AdditionalCategories::Geoscience
        } else if item == "History" {
            AdditionalCategories::History
        } else if item == "Humanities" {
            AdditionalCategories::Humanities
        } else if item == "ImageProcessing" {
            AdditionalCategories::ImageProcessing
        } else if item == "Literature" {
            AdditionalCategories::Literature
        } else if item == "Maps" {
            AdditionalCategories::Maps
        } else if item == "Math" {
            AdditionalCategories::Math
        } else if item == "NumericalAnalysis" {
            AdditionalCategories::NumericalAnalysis
        } else if item == "MedicalSoftware" {
            AdditionalCategories::MedicalSoftware
        } else if item == "Physics" {
            AdditionalCategories::Physics
        } else if item == "Robotics" {
            AdditionalCategories::Robotics
        } else if item == "Spirituality" {
            AdditionalCategories::Spirituality
        } else if item == "Sports" {
            AdditionalCategories::Sports
        } else if item == "ParallelComputing" {
            AdditionalCategories::ParallelComputing
        } else if item == "Amusement" {
            AdditionalCategories::Amusement
        } else if item == "Archiving" {
            AdditionalCategories::Archiving
        } else if item == "Compression" {
            AdditionalCategories::Compression
        } else if item == "Electronics" {
            AdditionalCategories::Electronics
        } else if item == "Emulator" {
            AdditionalCategories::Emulator
        } else if item == "Engineering" {
            AdditionalCategories::Engineering
        } else if item == "FileTools" {
            AdditionalCategories::FileTools
        } else if item == "FileManager" {
            AdditionalCategories::FileManager
        } else if item == "TerminalEmulator" {
            AdditionalCategories::TerminalEmulator
        } else if item == "Filesystem" {
            AdditionalCategories::Filesystem
        } else if item == "Monitor" {
            AdditionalCategories::Monitor
        } else if item == "Security" {
            AdditionalCategories::Security
        } else if item == "Accessibility" {
            AdditionalCategories::Accessibility
        } else if item == "Calculator" {
            AdditionalCategories::Calculator
        } else if item == "Clock" {
            AdditionalCategories::Clock
        } else if item == "TextEditor" {
            AdditionalCategories::TextEditor
        } else if item == "Documentation" {
            AdditionalCategories::Documentation
        } else if item == "Adult" {
            AdditionalCategories::Adult
        } else if item == "Core" {
            AdditionalCategories::Core
        } else if item == "KDE" {
            AdditionalCategories::Kde
        } else if item == "GNOME" {
            AdditionalCategories::Gnome
        } else if item == "XFCE" {
            AdditionalCategories::Xfce
        } else if item == "GTK" {
            AdditionalCategories::Gtk
        } else if item == "Qt" {
            AdditionalCategories::Qt
        } else if item == "Motif" {
            AdditionalCategories::Motif
        } else if item == "Java" {
            AdditionalCategories::Java
        } else if item == "ConsoleOnly" {
            AdditionalCategories::ConsoleOnly
        } else  {
            AdditionalCategories::Unknown
        }
    }
}
impl fmt::Display for AdditionalCategories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v:String = match *self {
            AdditionalCategories::Building => "Building".to_string(),
            AdditionalCategories::Debugger => "Debugger".to_string(),
            AdditionalCategories::Ide => "IDE".to_string(),
            AdditionalCategories::GuiDesigner => "GUIDesigner".to_string(),
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
            AdditionalCategories::Pda => "PDA".to_string(),
            AdditionalCategories::ProjectManagement => "ProjectManagement".to_string(),
            AdditionalCategories::Presentation => "Presentation".to_string(),
            AdditionalCategories::Spreadsheet => "Spreadsheet".to_string(),
            AdditionalCategories::WordProcessor => "WordProcessor".to_string(),
            AdditionalCategories::Graphics2D => "2DGraphics".to_string(),
            AdditionalCategories::VectorGraphics => "VectorGraphics".to_string(),
            AdditionalCategories::RasterGraphics => "RasterGraphics".to_string(),
            AdditionalCategories::Graphics3D => "3DGraphics".to_string(),
            AdditionalCategories::Scanning => "Scanning".to_string(),
            AdditionalCategories::Ocr => "OCR".to_string(),
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
            AdditionalCategories::Tv => "TV".to_string(),
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
            AdditionalCategories::Kde => "KDE".to_string(),
            AdditionalCategories::Gnome => "GNOME".to_string(),
            AdditionalCategories::Xfce => "XFCE".to_string(),
            AdditionalCategories::Gtk => "GTK".to_string(),
            AdditionalCategories::Qt => "Qt".to_string(),
            AdditionalCategories::Motif => "Motif".to_string(),
            AdditionalCategories::Java => "Java".to_string(),
            AdditionalCategories::ConsoleOnly => "ConsoleOnly".to_string(),
            AdditionalCategories::Unknown => "Unknown".to_string(),
        };
        write!(f, "{:?}", v)
    }
}

