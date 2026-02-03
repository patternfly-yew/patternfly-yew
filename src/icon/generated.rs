#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    strum_macros::EnumIter,
    strum_macros::EnumMessage,
    strum_macros::AsRefStr,
    strum_macros::IntoStaticStr,
)]
pub enum Icon {
    /// Indicates the ability to navigate to the first page of a multi-page data set
    AngleDoubleLeft,
    /// Indicates the ability to navigate to the last page of a multi-page data set
    AngleDoubleRight,
    /// Indicates expandable components such as accordions, progressive disclosures, or expandable lists are currently expanded. Clicking this will collapse the component.
    AngleDown,
    /// Indicates the ability to navigate to the previous page of a multi-page data set
    AngleLeft,
    /// Indicates expandable elements such as accordions, progressive disclosures, or expandable lists are currently collapsed. Clicking this will expand the element.
    AngleRight,
    /// Indicates expandable table rows (on mobile) are currently expanded. Clicking this will collapse the component.
    AngleUp,
    /// Represents "Ansible Tower"
    AnsibleTower,
    /// Represents status: an item (such as a VM) is down
    ArrowCircleDown,
    /// Represents status: an item (such as a VM) is up
    ArrowCircleUp,
    /// Indicates the availability of a sorting function in a table header
    ArrowsAltV,
    /// Indicates the ability to take an action or navigate to another page. Is paired with text
    ArrowRight,
    /// Represents status: an item needs rebalancing
    BalanceScale,
    /// Represents status: an item is disabled, canceled, terminated or is not ready
    Ban,
    /// Indicates the ability to collapse a navigation menu
    Bars,
    /// Indicates the ability to open a notification drawer.
    Bell,
    /// Represents status: attention
    AttentionBell,
    /// Represents status: there is a bug present
    Bug,
    /// Indicates a date picker function is available
    #[cfg(feature = "icons-far")]
    OutlinedCalendarAlt,
    /// Indicates the ability to acces option panels for components like drop-downs, filters and page ranges
    CaretDown,
    /// Represents status: Indicates a switch toggle is in the enabled position
    Check,
    /// Indicates the ability to commit edited changes
    CheckCircle,
    /// Represents orders or tasks
    ClipboardCheck,
    /// Represents a time interval
    #[cfg(feature = "icons-far")]
    OutlinedClock,
    /// Represents code
    Code,
    /// Represents code branch
    CodeBranch,
    /// Indicates availability of configurable settings
    Cog,
    /// Indicates the ability to manage columns for a table view
    Columns,
    /// Indicates availability of commenting
    #[cfg(feature = "icons-far")]
    OutlinedComments,
    /// Indicates the ability to compress an item. Should toggle with fa-expand
    Compress,
    /// Indicates the ability to compress an item (alt concept). Should toggle with fa-expand-arrows-alt
    CompressArrowsAlt,
    /// Indicates the availability of a copy to clipboard function
    Copy,
    /// Represents potential critical impact on a system or cluster
    CriticalRisk,
    /// Represents a container
    Cube,
    /// Represents a Kubernetes pod(s)
    Cubes,
    /// Represents a database
    Database,
    /// Represents a desktop, workstation or terminal
    Desktop,
    /// Indicates a download function is available
    Download,
    /// Indicates a contextual menu of actions or additional actions is available
    EllipsisV,
    /// Represents alert status: danger, major error or critical error
    ExclamationCircle,
    /// Represents alert status: warning
    ExclamationTriangle,
    /// Indicates the ability to expand an item. Should toggle with fa-compress
    Expand,
    /// Indicates the ability to expand an item (alt concept). Should toggle with fa-compress-arrows-alt
    ExpandArrowsAlt,
    /// Indicates the link navigates to an external site
    ExternalLinkAlt,
    /// Indicates the content of a component is currently hidden but can be revealed
    Eye,
    /// Indicates the content of a component is revealed but can be hidden
    EyeSlash,
    /// Represents a file type
    File,
    /// Indicates the ability to filter search results or datasets
    Filter,
    /// Represents a message.
    Flag,
    /// Represents a collapsed hierarchical group.
    Folder,
    /// Represents an expanded hierarchical group.
    FolderOpen,
    /// Indicates the ability to move a vertically-oriented component via drag and drop
    GripHorizontal,
    /// Indicates the ability to move a horizontally-oriented component via drag and drop
    GripVertical,
    /// Represents status: restarting
    History,
    /// Indicates the ability to undo an a step in a historical log
    Undo,
    /// Represents a single node or host
    OutlinedHdd,
    /// Indicates a link to a default/home page
    Home,
    /// Represents alert status: information
    InfoCircle,
    /// Represents an SSH key or similar security concepts
    Key,
    /// Represents data view content in a list format.
    List,
    /// Represents status: locked
    Lock,
    /// Represents status: unlocked
    LockOpen,
    /// Represents the largest-to-smallest, highest-to-lowest or last-to-first (descending) sort order for any data type in a data table column. Clicking this will toggle the sort to ascending.
    LongArrowAltDown,
    /// Represents the smallest-to-largest, lowest-to-highest or first-to-last (ascending) sort order for any data type in a data table column. Clicking this will toggle the sort to descending.
    LongArrowAltUp,
    /// Represents a locale
    MapMarker,
    /// Represents the memory on a device
    Memory,
    /// Represents the CPU of a device
    Microchip,
    /// Indicates the ability to remove an item
    Minus,
    /// Indicates the ability to remove an item (alt concept)
    MinusCircle,
    /// Indicates the ability to pause. Should toggle with fa-play
    Pause,
    /// Represents status: an interruption and/or stoppage of a process
    PauseCircle,
    /// Indicates the ability to edit an item
    PencilAlt,
    /// Indicates the ability to add an item; not for creating completely new objects (see pficon-circle-add)
    Plus,
    /// Indicates the ability to create an item
    PlusCircle,
    /// Indicates the ability to create an item. Use this if there are many instances of this icon in a UI (data list, table, etc) to reduce visual noise.
    AddCircleO,
    /// Represents status: powered on
    PowerOff,
    /// Represents status: powered off
    Off,
    /// Represents brand: OpenShift
    Openshift,
    /// Represents brand: OpenStack
    Openstack,
    /// Indicates the ability to start or resume. Should toggle with fa-pause
    Play,
    /// Indicates the availability of a print function
    Print,
    /// Indicates the availability of contextual help
    #[cfg(feature = "icons-far")]
    OutlinedQuestionCircle,
    /// Indicates the availability of a help system in the masthead
    QuestionCircle,
    /// Indicates the ability to refresh. Please use the animated spinner to indicate that something is “loading” or in the middle of processing
    Redo,
    /// Indicates that that user may perform a search
    Search,
    /// Indicates the ability to zoom out
    SearchMinus,
    /// Indicates the ability to zoom in
    SearchPlus,
    /// Indicates the ability to share via various methods with others
    ShareSquare,
    /// Represents the largest-to-smallest, highest-to-lowest or last-to-first (descending) sort order for any data type.
    SortAmountDown,
    /// Represents the smallest-to-largest, lowest-to-highest or first-to-last (ascending) sort order for any data type.
    SortAmountDownAlt,
    /// Indicates the availability of a sync action
    SyncAlt,
    /// Indicates the abiltiy to access or create a set of tags
    Tag,
    /// Represents data view content in a table format
    Table,
    /// Represents data view content in a dashboard
    TachometerAlt,
    /// Indicates the ability to open a task drawer. Also can represent tasks or activity.
    Task,
    /// Represents data view content in a small card format
    Th,
    /// Represents data view content in a large card format
    ThLarge,
    /// Indicates the ability to pin an item
    Thumbtack,
    /// Indicates the ability to close a modal or other panel. Also indicates the ability to clear existing data, such as filter criteria or labels
    Times,
    /// Indicates the ability to close the about modal
    TimesCircle,
    /// Indicates the ability to delete
    Trash,
    /// Indicates an upload function is available
    Upload,
    /// Represents a user (in a dataset, paired with a username).
    User,
    /// Represents multiple users, a user grouping or project
    Users,
    /// Indicates the ability to open link in a new window
    #[cfg(feature = "icons-far")]
    OutlinedWindowRestore,
    /// Represents status: in preparation for maintenance
    Wrench,
    /// Represents brand: GitHub
    #[cfg(feature = "icons-fab")]
    Github,
    /// Represents brand: GitLab
    #[cfg(feature = "icons-fab")]
    Gitlab,
    /// Represents brand: Google
    #[cfg(feature = "icons-fab")]
    Google,
    /// Represents brand: Stack Overflow
    #[cfg(feature = "icons-fab")]
    StackOverflow,
    /// Represents brand: Facebook
    #[cfg(feature = "icons-fab")]
    FacebookSquare,
    /// Represents brand: Twitter
    #[cfg(feature = "icons-fab")]
    Twitter,
    /// Represents brand: Windows
    #[cfg(feature = "icons-fab")]
    Windows,
    /// Represents brand: LinkedIn
    #[cfg(feature = "icons-fab")]
    Linkedin,
    /// Represents brand: Linux
    #[cfg(feature = "icons-fab")]
    Linux,
    /// Represents brand: Dropbox
    #[cfg(feature = "icons-fab")]
    Dropbox,
    /// Represents brand: Drupal
    #[cfg(feature = "icons-fab")]
    Drupal,
    /// Represents brand: js
    #[cfg(feature = "icons-fab")]
    Js,
    /// Represents an item is asleep or in power suspended mode
    Asleep,
    /// Represents a process-automation object
    Automation,
    /// Represents a blueprint
    Blueprint,
    /// Represents a build
    Build,
    /// Represents a builder image
    BuilderImage,
    /// Represents a package; used in Satellite, Cockpit, and Composer to indicate a generic package or rpm
    Bundle,
    /// Indicates the availability of a catalog or library
    Catalog,
    /// Represents cloud security
    CloudSecurity,
    /// Represents a cloud tenant
    CloudTenant,
    /// Represents a cluster or server
    Cluster,
    /// Represents an item's power is on and is “up”; this is a more active alternative to “pficon-on”
    Connected,
    /// Represents a data processor
    DataProcessor,
    /// Represents a data sink
    DataSink,
    /// Represents a data source
    DataSource,
    /// Volume replication is degraded
    Degraded,
    /// Represents an item's power is off and is “down”; this is a more active alternative to “pficon-off”
    Disconnected,
    /// Represents a domain
    Domain,
    /// Represents status: enhancement advisory is present
    Enhancement,
    /// Represents an enterprise
    Enterprise,
    /// Indicates the ability to export a file or other data
    Export,
    /// Represents a flavor
    Flavor,
    /// Represents an image
    Image,
    /// Indicates the ability to import a file or other data
    Import,
    /// Represents running a determinite action
    InProgress,
    /// Represents an infrastructure
    Infrastructure,
    /// Represents an integration of two or more objects
    Integration,
    /// Represents an item such as a VM is currently migrating
    Migration,
    /// Represents middleware
    Middleware,
    /// Represents a module
    Module,
    /// Represents monitoring
    Monitoring,
    /// Indicates a multicluster object
    Multicluster,
    /// Represents network
    Network,
    /// Represents network range
    NetworkRange,
    /// Represents status: running
    Running,
    /// Open or close a drawer
    OpenDrawerRight,
    /// Indicates the ability to optimize an item or a process
    Optimize,
    /// Represents a package
    Package,
    /// Represents status: pending; currently waiting on contingencies
    Pending,
    /// Represents status: private; cannot access with current credentials
    Private,
    /// Represents a port or route
    Port,
    /// Represents process automation
    ProcessAutomation,
    /// Represents a region
    Regions,
    /// Represents a registry
    Registry,
    /// Represents a replicator
    Replicator,
    /// Represents a repository
    Repository,
    /// Represents a resource pool
    ResourcePool,
    /// Represents status: is empty
    ResourcesEmpty,
    /// Represents status: is almost empty
    ResourcesAlmostEmpty,
    /// Represents status: is almost full
    ResourcesAlmostFull,
    /// Represents status: is full
    ResourcesFull,
    /// Represents a route
    Route,
    /// Represents brand: Satellite
    Satellite,
    /// Indicates the ability to save a file or other object
    Save,
    /// Represents status: security advisory is present
    Security,
    /// Represents a server group
    ServerGroup,
    /// Represents a Kubernetes service
    Service,
    /// Represents services
    Services,
    /// Indicates availability of a catalog/library to browse
    ServiceCatalog,
    /// Indicates a storage domain
    StorageDomain,
    /// Represents a template; includes contents or instructions used to generate one or more instances of a final output
    Template,
    /// Represents a tenant
    Tenant,
    /// Represents data view content in a topology format
    Topology,
    /// Represents status: downward trend
    TrendDown,
    /// Represents status: upward trend
    TrendUp,
    /// Represents status: unknown
    Unknown,
    /// Represents a vcenter
    Vcenter,
    /// Represents a virtual machine
    VirtualMachine,
    /// Represents a volume
    Volume,
    /// Represents a zone; a grouping of servers based on geographic location, network location, or function
    Zone,
    /// Indicates the ability to acces option panels for components like drop-downs, filters and page ranges
    CaretUp,
}

impl crate::core::AsClasses for Icon {
    fn extend_classes(&self, classes: &mut yew::prelude::Classes) {
        match self {
            Self::AngleDoubleLeft => classes.extend(super::fas("fa-angle-double-left")),
            Self::AngleDoubleRight => classes.extend(super::fas("fa-angle-double-right")),
            Self::AngleDown => classes.extend(super::fas("fa-angle-down")),
            Self::AngleLeft => classes.extend(super::fas("fa-angle-left")),
            Self::AngleRight => classes.extend(super::fas("fa-angle-right")),
            Self::AngleUp => classes.extend(super::fas("fa-angle-up")),
            Self::AnsibleTower => classes.extend(super::pf("pf-v6-pficon-ansible-tower")),
            Self::ArrowCircleDown => classes.extend(super::fas("fa-arrow-circle-down")),
            Self::ArrowCircleUp => classes.extend(super::fas("fa-arrow-circle-up")),
            Self::ArrowsAltV => classes.extend(super::fas("fa-arrows-alt-v")),
            Self::ArrowRight => classes.extend(super::fas("fa-arrow-right")),
            Self::BalanceScale => classes.extend(super::fas("fa-balance-scale")),
            Self::Ban => classes.extend(super::fas("fa-ban")),
            Self::Bars => classes.extend(super::fas("fa-bars")),
            Self::Bell => classes.extend(super::fas("fa-bell")),
            Self::AttentionBell => classes.extend(super::pf("pf-v6-pficon-attention-bell")),
            Self::Bug => classes.extend(super::fas("fa-bug")),
            #[cfg(feature = "icons-far")]
            Self::OutlinedCalendarAlt => classes.extend(super::far("fa-calendar-alt")),
            Self::CaretDown => classes.extend(super::fas("fa-caret-down")),
            Self::Check => classes.extend(super::fas("fa-check")),
            Self::CheckCircle => classes.extend(super::fas("fa-check-circle")),
            Self::ClipboardCheck => classes.extend(super::fas("fa-clipboard-check")),
            #[cfg(feature = "icons-far")]
            Self::OutlinedClock => classes.extend(super::far("fa-clock")),
            Self::Code => classes.extend(super::fas("fa-code")),
            Self::CodeBranch => classes.extend(super::fas("fa-code-branch")),
            Self::Cog => classes.extend(super::fas("fa-cog")),
            Self::Columns => classes.extend(super::fas("fa-columns")),
            #[cfg(feature = "icons-far")]
            Self::OutlinedComments => classes.extend(super::far("fa-comments")),
            Self::Compress => classes.extend(super::fas("fa-compress")),
            Self::CompressArrowsAlt => classes.extend(super::fas("fa-compress-arrows-alt")),
            Self::Copy => classes.extend(super::fas("fa-copy")),
            Self::CriticalRisk => classes.extend(super::pf("pf-v6-pficon-critical-risk")),
            Self::Cube => classes.extend(super::fas("fa-cube")),
            Self::Cubes => classes.extend(super::fas("fa-cubes")),
            Self::Database => classes.extend(super::fas("fa-database")),
            Self::Desktop => classes.extend(super::fas("fa-desktop")),
            Self::Download => classes.extend(super::fas("fa-download")),
            Self::EllipsisV => classes.extend(super::fas("fa-ellipsis-v")),
            Self::ExclamationCircle => classes.extend(super::fas("fa-exclamation-circle")),
            Self::ExclamationTriangle => classes.extend(super::fas("fa-exclamation-triangle")),
            Self::Expand => classes.extend(super::fas("fa-expand")),
            Self::ExpandArrowsAlt => classes.extend(super::fas("fa-expand-arrows-alt")),
            Self::ExternalLinkAlt => classes.extend(super::fas("fa-external-link-alt")),
            Self::Eye => classes.extend(super::fas("fa-eye")),
            Self::EyeSlash => classes.extend(super::fas("fa-eye-slash")),
            Self::File => classes.extend(super::fas("fa-file")),
            Self::Filter => classes.extend(super::fas("fa-filter")),
            Self::Flag => classes.extend(super::fas("fa-flag")),
            Self::Folder => classes.extend(super::fas("fa-folder")),
            Self::FolderOpen => classes.extend(super::fas("fa-folder-open")),
            Self::GripHorizontal => classes.extend(super::fas("fa-grip-horizontal")),
            Self::GripVertical => classes.extend(super::fas("fa-grip-vertical")),
            Self::History => classes.extend(super::fas("fa-history")),
            Self::Undo => classes.extend(super::fas("fa-undo")),
            Self::OutlinedHdd => classes.extend(super::fas("fa-hdd")),
            Self::Home => classes.extend(super::fas("fa-home")),
            Self::InfoCircle => classes.extend(super::fas("fa-info-circle")),
            Self::Key => classes.extend(super::fas("fa-key")),
            Self::List => classes.extend(super::fas("fa-list")),
            Self::Lock => classes.extend(super::fas("fa-lock")),
            Self::LockOpen => classes.extend(super::fas("fa-lock-open")),
            Self::LongArrowAltDown => classes.extend(super::fas("fa-long-arrow-alt-down")),
            Self::LongArrowAltUp => classes.extend(super::fas("fa-long-arrow-alt-up")),
            Self::MapMarker => classes.extend(super::fas("fa-map-marker")),
            Self::Memory => classes.extend(super::fas("fa-memory")),
            Self::Microchip => classes.extend(super::fas("fa-microchip")),
            Self::Minus => classes.extend(super::fas("fa-minus")),
            Self::MinusCircle => classes.extend(super::fas("fa-minus-circle")),
            Self::Pause => classes.extend(super::fas("fa-pause")),
            Self::PauseCircle => classes.extend(super::fas("fa-pause-circle")),
            Self::PencilAlt => classes.extend(super::fas("fa-pencil-alt")),
            Self::Plus => classes.extend(super::fas("fa-plus")),
            Self::PlusCircle => classes.extend(super::fas("fa-plus-circle")),
            Self::AddCircleO => classes.extend(super::pf("pf-v6-pficon-add-circle-o")),
            Self::PowerOff => classes.extend(super::fas("fa-power-off")),
            Self::Off => classes.extend(super::pf("pf-v6-pficon-off")),
            Self::Openshift => classes.extend(super::pf("pf-v6-pficon-openshift")),
            Self::Openstack => classes.extend(super::pf("pf-v6-pficon-openstack")),
            Self::Play => classes.extend(super::fas("fa-play")),
            Self::Print => classes.extend(super::fas("fa-print")),
            #[cfg(feature = "icons-far")]
            Self::OutlinedQuestionCircle => {
                classes.extend(super::far("fa-question-circle (outlined)"))
            }
            Self::QuestionCircle => classes.extend(super::fas("fa-question-circle")),
            Self::Redo => classes.extend(super::fas("fa-redo")),
            Self::Search => classes.extend(super::fas("fa-search")),
            Self::SearchMinus => classes.extend(super::fas("fa-search-minus")),
            Self::SearchPlus => classes.extend(super::fas("fa-search-plus")),
            Self::ShareSquare => classes.extend(super::fas("fa-share-square")),
            Self::SortAmountDown => classes.extend(super::fas("fa-sort-amount-down")),
            Self::SortAmountDownAlt => classes.extend(super::fas("fa-sort-amount-down-alt")),
            Self::SyncAlt => classes.extend(super::fas("fa-sync-alt")),
            Self::Tag => classes.extend(super::fas("fa-tag")),
            Self::Table => classes.extend(super::fas("fa-table")),
            Self::TachometerAlt => classes.extend(super::fas("fa-tachometer-alt")),
            Self::Task => classes.extend(super::pf("pf-v6-pficon-task")),
            Self::Th => classes.extend(super::fas("fa-th")),
            Self::ThLarge => classes.extend(super::fas("fa-th-large")),
            Self::Thumbtack => classes.extend(super::fas("fa-thumbtack")),
            Self::Times => classes.extend(super::fas("fa-times")),
            Self::TimesCircle => classes.extend(super::fas("fa-times-circle")),
            Self::Trash => classes.extend(super::fas("fa-trash")),
            Self::Upload => classes.extend(super::fas("fa-upload")),
            Self::User => classes.extend(super::fas("fa-user")),
            Self::Users => classes.extend(super::fas("fa-users")),
            #[cfg(feature = "icons-far")]
            Self::OutlinedWindowRestore => classes.extend(super::far("fa-window-restore")),
            Self::Wrench => classes.extend(super::fas("fa-wrench")),
            #[cfg(feature = "icons-fab")]
            Self::Github => classes.extend(super::fab("fa-github")),
            #[cfg(feature = "icons-fab")]
            Self::Gitlab => classes.extend(super::fab("fa-gitlab")),
            #[cfg(feature = "icons-fab")]
            Self::Google => classes.extend(super::fab("fa-google")),
            #[cfg(feature = "icons-fab")]
            Self::StackOverflow => classes.extend(super::fab("fa-stack-overflow")),
            #[cfg(feature = "icons-fab")]
            Self::FacebookSquare => classes.extend(super::fab("fa-facebook-square")),
            #[cfg(feature = "icons-fab")]
            Self::Twitter => classes.extend(super::fab("fa-twitter")),
            #[cfg(feature = "icons-fab")]
            Self::Windows => classes.extend(super::fab("fa-windows")),
            #[cfg(feature = "icons-fab")]
            Self::Linkedin => classes.extend(super::fab("fa-linkedin")),
            #[cfg(feature = "icons-fab")]
            Self::Linux => classes.extend(super::fab("fa-linux")),
            #[cfg(feature = "icons-fab")]
            Self::Dropbox => classes.extend(super::fab("fa-dropbox")),
            #[cfg(feature = "icons-fab")]
            Self::Drupal => classes.extend(super::fab("fa-drupal")),
            #[cfg(feature = "icons-fab")]
            Self::Js => classes.extend(super::fab("fa-js")),
            Self::Asleep => classes.extend(super::pf("pf-v6-pficon-asleep")),
            Self::Automation => classes.extend(super::pf("pf-v6-pficon-automation")),
            Self::Blueprint => classes.extend(super::pf("pf-v6-pficon-blueprint")),
            Self::Build => classes.extend(super::pf("pf-v6-pficon-build")),
            Self::BuilderImage => classes.extend(super::pf("pf-v6-pficon-builder-image")),
            Self::Bundle => classes.extend(super::pf("pf-v6-pficon-bundle")),
            Self::Catalog => classes.extend(super::pf("pf-v6-pficon-catalog")),
            Self::CloudSecurity => classes.extend(super::pf("pf-v6-pficon-cloud-security")),
            Self::CloudTenant => classes.extend(super::pf("pf-v6-pficon-cloud-tenant")),
            Self::Cluster => classes.extend(super::pf("pf-v6-pficon-cluster")),
            Self::Connected => classes.extend(super::pf("pf-v6-pficon-connected")),
            Self::DataProcessor => classes.extend(super::pf("pf-v6-pficon-data-processor")),
            Self::DataSink => classes.extend(super::pf("pf-v6-pficon-data-sink")),
            Self::DataSource => classes.extend(super::pf("pf-v6-pficon-data-source")),
            Self::Degraded => classes.extend(super::pf("pf-v6-pficon-degraded")),
            Self::Disconnected => classes.extend(super::pf("pf-v6-pficon-disconnected")),
            Self::Domain => classes.extend(super::pf("pf-v6-pficon-domain")),
            Self::Enhancement => classes.extend(super::pf("pf-v6-pficon-enhancement")),
            Self::Enterprise => classes.extend(super::pf("pf-v6-pficon-enterprise")),
            Self::Export => classes.extend(super::pf("pf-v6-pficon-export")),
            Self::Flavor => classes.extend(super::pf("pf-v6-pficon-flavor")),
            Self::Image => classes.extend(super::fas("fa-image")),
            Self::Import => classes.extend(super::pf("pf-v6-pficon-import")),
            Self::InProgress => classes.extend(super::pf("pf-v6-pficon-in-progress")),
            Self::Infrastructure => classes.extend(super::pf("pf-v6-pficon-infrastructure")),
            Self::Integration => classes.extend(super::pf("pf-v6-pficon-integration")),
            Self::Migration => classes.extend(super::pf("pf-v6-pficon-migration")),
            Self::Middleware => classes.extend(super::pf("pf-v6-pficon-middleware")),
            Self::Module => classes.extend(super::pf("pf-v6-pficon-module")),
            Self::Monitoring => classes.extend(super::pf("pf-v6-pficon-monitoring")),
            Self::Multicluster => classes.extend(super::pf("pf-v6-pficon-multicluster")),
            Self::Network => classes.extend(super::pf("pf-v6-pficon-network")),
            Self::NetworkRange => classes.extend(super::pf("pf-v6-pficon-pficon-network-range")),
            Self::Running => classes.extend(super::pf("pf-v6-pficon-running")),
            Self::OpenDrawerRight => classes.extend(super::pf("pf-v6-pficon-open-drawer-right")),
            Self::Optimize => classes.extend(super::pf("pf-v6-pficon-optimize")),
            Self::Package => classes.extend(super::pf("pf-v6-pficon-package")),
            Self::Pending => classes.extend(super::pf("pf-v6-pficon-pending")),
            Self::Private => classes.extend(super::pf("pf-v6-pficon-private")),
            Self::Port => classes.extend(super::pf("pf-v6-pficon-port")),
            Self::ProcessAutomation => classes.extend(super::pf("pf-v6-pficon-process-automation")),
            Self::Regions => classes.extend(super::pf("pf-v6-pficon-regions")),
            Self::Registry => classes.extend(super::pf("pf-v6-pficon-registry")),
            Self::Replicator => classes.extend(super::pf("pf-v6-pficon-replicator")),
            Self::Repository => classes.extend(super::pf("pf-v6-pficon-repository")),
            Self::ResourcePool => classes.extend(super::pf("pf-v6-pficon-resource-pool")),
            Self::ResourcesEmpty => classes.extend(super::pf("pf-v6-pficon-resources-empty")),
            Self::ResourcesAlmostEmpty => {
                classes.extend(super::pf("pf-v6-pficon-resources-almost-empty"))
            }
            Self::ResourcesAlmostFull => {
                classes.extend(super::pf("pf-v6-pficon-resources-almost-full"))
            }
            Self::ResourcesFull => classes.extend(super::pf("pf-v6-pficon-resources-full")),
            Self::Route => classes.extend(super::fas("fa-route")),
            Self::Satellite => classes.extend(super::pf("pf-v6-pficon-pficon-satellite")),
            Self::Save => classes.extend(super::pf("pf-v6-pficon-save")),
            Self::Security => classes.extend(super::pf("pf-v6-pficon-security")),
            Self::ServerGroup => classes.extend(super::pf("pf-v6-pficon-server-group")),
            Self::Service => classes.extend(super::pf("pf-v6-pficon-service")),
            Self::Services => classes.extend(super::pf("pf-v6-pficon-services")),
            Self::ServiceCatalog => classes.extend(super::pf("pf-v6-pficon-service-catalog")),
            Self::StorageDomain => classes.extend(super::pf("pf-v6-pficon-storage-domain")),
            Self::Template => classes.extend(super::pf("pf-v6-pficon-pficon-template")),
            Self::Tenant => classes.extend(super::pf("pf-v6-pficon-tenant")),
            Self::Topology => classes.extend(super::pf("pf-v6-pficon-topology")),
            Self::TrendDown => classes.extend(super::pf("pf-v6-pficon-trend-down")),
            Self::TrendUp => classes.extend(super::pf("pf-v6-pficon-trend-up")),
            Self::Unknown => classes.extend(super::pf("pf-v6-pficon-unknown")),
            Self::Vcenter => classes.extend(super::pf("pf-v6-pficon-pficon-vcenter")),
            Self::VirtualMachine => classes.extend(super::pf("pf-v6-pficon-virtual-machine")),
            Self::Volume => classes.extend(super::pf("pf-v6-pficon-volume")),
            Self::Zone => classes.extend(super::pf("pf-v6-pficon-zone")),
            Self::CaretUp => classes.extend(super::fas("fa-caret-up")),
        }
    }
}
