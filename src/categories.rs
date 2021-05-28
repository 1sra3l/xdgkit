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
#[derive(Debug)]
pub enum Categories {
    /// Application for presenting, creating, or processing multimedia (audio/video)
    AudioVideo,
    /// An audio application *Desktop entry must include AudioVideo as well*
    Audio,
    /// A video application *Desktop entry must include AudioVideo as well Development An application for development*
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
    /// Settings applications *Entries may appear in a separate menu or as part of a "Control Center"*
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
    write!(f, "{:?}", self)
    // or, alternatively:
    // fmt::Debug::fmt(self, f)
    }
}
#[allow(dead_code)]
impl Categories {
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
pub fn get_additional(item:String) -> AdditionalCategories{
    if item == "Unknown" {
        return AdditionalCategories::Unknown
    }
    AdditionalCategories::Unknown
}
/// The `Related Category` lists one or more categories that are suggested to be used in conjunction with the Additional Category. If there is no `Related Category`, the listed Category can be used with any Main Category.
/// `Related Category` is noted below as `CategoryName`
///
/// The `enum` below describes Additional Categories.
#[allow(dead_code)]
#[derive(Debug)]
pub enum AdditionalCategories {
/*TODO format doc corretly
    Building,/// A tool to build applications `Development`
    Debugger,/// A tool to debug applications `Development`
    IDE,/// IDE application Development
    GUIDesigner,/// A GUI designer application `Development`
    Profiling,/// A profiling tool `Development`
    RevisionControl,/// Applications like cvs or subversion `Development` 
    Translation,/// A translation tool `Development`
    Calendar,/// Calendar application `Office`
    ContactManagement,/// E.g. an address book `Office`
    Database,/// Application to manage a database `Office` or `Development` or `AudioVideo`
    Dictionary,/// A dictionary `Office` or `TextTools`
    Chart,/// Chart application `Office`
    Email,/// Email application `Office` or `Network`
    Finance,/// Application to manage your finance `Office`
    FlowChart,/// A flowchart application `Office`
    PDA,/// Tool to manage your PDA `Office`
    ProjectManagement,/// Project management application `Office` or `Development`
    Presentation,/// Presentation software `Office`
    Spreadsheet,/// A spreadsheet `Office`
    WordProcessor,/// A word processor `Office`
    2DGraphics,/// 2D based graphical application `Graphics`
    VectorGraphics,/// Application for viewing, creating, or processing vector graphics `Graphics;2DGraphics`
    RasterGraphics,/// Application for viewing, creating, or processing raster (bitmap) graphics `Graphics;2DGraphics`
    3DGraphics,/// Application for viewing, creating, or processing 3-D graphics `Graphics`
    Scanning,/// Tool to scan a file/text `Graphics`
    OCR,/// Optical character recognition application `Graphics;Scanning`
    Photography,/// Camera tools, etc. `Graphics` or `Office`
    Publishing,/// Desktop Publishing applications and Color Management tools `Graphics` or `Office`
    Viewer,/// Tool to view e.g. a graphic or pdf file `Graphics` or `Office`
    TextTools,/// A text tool utility `Utility`
    DesktopSettings,/// Configuration tool for the GUI `Settings`
    HardwareSettings,/// A tool to manage hardware components, like sound cards, video cards or printers `Settings`
    Printing,/// A tool to manage printers `HardwareSettings;Settings`
    PackageManager,/// A package manager application `Settings`
    Dialup,/// A dial-up program `Network`
    InstantMessaging,/// An instant messaging client `Network`
    Chat,/// A chat client `Network`
    IRCClient,/// An IRC client `Network`
    Feed,/// RSS, podcast and other subscription based contents `Network`
    FileTransfer,/// Tools like FTP or P2P programs `Network`
    HamRadio,/// HAM radio software `Network` or `Audio`
    News,/// A news reader or a news ticker `Network`
    P2P,/// A P2P program `Network`
    RemoteAccess,/// A tool to remotely manage your PC `Network`
    Telephony,/// Telephony via PC `Network`
    TelephonyTools,/// Telephony tools, to dial a number, manage PBX, ... `Utility`
    VideoConference,/// Video Conference software `Network`
    WebBrowser,/// A web browser `Network`
    WebDevelopment,/// A tool for web developers `Network` or `Development`
    Midi,/// An app related to MIDI `AudioVideo;Audio` 
    Mixer,/// Just a mixer `AudioVideo;Audio`
    Sequencer,/// A sequencer `AudioVideo;Audio`
    Tuner,/// A tuner `AudioVideo;Audio`
    TV,/// A TV application `AudioVideo;Video`
    AudioVideoEditing,/// Application to edit audio/video files `Audio` or `Video` or`Audio`
    VideoPlayer,/// Application to play audio/video files `Audio` or `Video` or`AudioVideo`
    Recorder,/// Application to record audio/video files `Audio` or `Video` or `AudioVideo`
    DiscBurning,/// Application to burn a disc `AudioVideo`
    ActionGame,/// An action game `Game`
    AdventureGame,/// Adventure style game `Game`
    ArcadeGame,/// Arcade style game `Game`
    BoardGame,/// A board game `Game`
    BlocksGame,/// Falling blocks game `Game`
    CardGame,/// A card game `Game`
    KidsGame,/// A game for kids `Game`
    LogicGame,/// Logic games like puzzles, etc `Game` 
    RolePlaying,/// A role playing game `Game`
    Shooter,/// A shooter game `Game`
    Simulation,/// A simulation game `Game`
    SportsGame,/// A sports game `Game`
    StrategyGame,/// A strategy game `Game`
    Art,/// Software to teach arts `Education` or `Science`` 
    Construction,///``Education` or `Science``
    Music,/// Musical software `AudioVideo` or `Education`
    Languages,/// Software to learn foreign languages `Education` or `Science`
    ArtificialIntelligence,/// Artificial Intelligence software `Education` or `Science`
    Astronomy,/// Astronomy software `Education` or `Science`
    Biology,/// Biology software `Education` or `Science`
    Chemistry,/// Chemistry software `Education` or `Science`
    ComputerScience,/// ComputerSience software `Education` or `Science`
    DataVisualization,/// Data visualization software `Education` or `Science`
    Economy,/// Economy software `Education` or `Science`
    Electricity,/// Electricity software `Education` or `Science`
    Geography,/// Geography software `Education` or `Science`
    Geology,/// Geology software `Education` or `Science`
    Geoscience,/// Geoscience software, GIS `Education` or `Science`
    History,/// History software `Education` or `Science`
    Humanities,/// Software for philosophy, psychology and other humanities `Education` or `Science`
    ImageProcessing,/// Image Processing software `Education` or `Science`
    Literature,/// Literature software `Education` or `Science`
    Maps,/// Sofware for viewing maps, navigation, mapping, GPS `Education` or `Science` or `Utility` 
    Math,/// Math software `Education` or `Science` 
    NumericalAnalysis,/// Numerical analysis software `Education;Math` or `Science;Math` 
    MedicalSoftware,/// Medical software `Education` or `Science` 
    Physics,/// Physics software `Education` or `Science`
    Robotics,/// Robotics software `Education` or `Science`
    Spirituality,/// Religious and spiritual software, theology `Education` or `Science` or `Utility`
    Sports,/// Sports software `Education` or `Science`
    ParallelComputing,/// Parallel computing software `Education;ComputerScience` or `Science;ComputerScience`
    Amusement,/// A simple amusement
    Archiving,/// A tool to archive/backup data `Utility`
    Compression,/// A tool to manage compressed data/archives `Utility;Archiving`
    Electronics,/// Electronics software, e.g. a circuit designer
    Emulator,/// Emulator of another platform, such as a DOS emulator `System` or `Game`
    Engineering,/// Engineering software, e.g. CAD programs
    FileTools,/// A file tool utility `Utility` or `System`
    FileManager,/// A file manager `System;FileTools`
    TerminalEmulator,/// A terminal emulator application `System`
    Filesystem,/// A file system tool `System`
    Monitor,/// Monitor application/applet that monitors some resource or activity `System` or `Network`
    Security,/// A security tool `Settings` or `System`
    Accessibility,/// Accessibility `Settings` or `Utility`
    Calculator,/// A calculator `Utility`
    Clock,/// A clock application/applet `Utility`
    TextEditor,/// A text editor `Utility`
    Documentation,/// Help or documentation
    Adult,/// Application handles adult or explicit material
    Core,/// Important application, core to the desktop such as a file manager or a help browser
    KDE,/// Application based on KDE libraries `QT`
    GNOME,/// Application based on GNOME libraries `GTK`
    XFCE,/// Application based on XFCE libraries `GTK`
    GTK,/// Application based on GTK+ libraries
    Qt,/// Application based on Qt libraries
    Motif,/// Application based on Motif libraries
    Java,/// Application based on Java GUI libraries, such as AWT or Swing
    ConsoleOnly,/// Application that only works inside a terminal (text-based or command line application)
*/
    /// This is for random people making whatever they want... `Unknown` is similar to Desktop Entry's `type`
    Unknown,
}

impl fmt::Display for AdditionalCategories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    // or, alternatively:
    // fmt::Debug::fmt(self, f)
    }
}
