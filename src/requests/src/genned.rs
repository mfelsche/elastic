// This code is automatically generated
//
pub mod endpoints {
    use super::http::*;
    use super::params::*;

    #[derive(Debug, PartialEq, Clone)]
    enum IndicesCloseUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesCloseUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesCloseUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_close");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesCloseRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesCloseRequest<'a, B> {
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesCloseRequest {
                url: IndicesCloseUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesCloseRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum DeleteScriptUrlParams<'a> {
        LangId(Lang<'a>, Id<'a>),
    }
    impl<'a> DeleteScriptUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                DeleteScriptUrlParams::LangId(ref lang, ref id) => {
                    let mut url = String::with_capacity(11usize + lang.len() + id.len());
                    url.push_str("/_scripts/");
                    url.push_str(lang.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct DeleteScriptRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> DeleteScriptRequest<'a> {
        pub fn for_lang_id<ILang, IId>(lang: ILang, id: IId) -> Self
        where
            ILang: Into<Lang<'a>>,
            IId: Into<Id<'a>>,
        {
            DeleteScriptRequest {
                url: DeleteScriptUrlParams::LangId(lang.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for DeleteScriptRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum TermvectorsUrlParams<'a> {
        IndexType(Index<'a>, Type<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> TermvectorsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                TermvectorsUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(15usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_termvectors");
                    Url::from(url)
                }
                TermvectorsUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(16usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_termvectors");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct TermvectorsRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> TermvectorsRequest<'a, B> {
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            TermvectorsRequest {
                url: TermvectorsUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            TermvectorsRequest {
                url: TermvectorsUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for TermvectorsRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum FieldStatsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> FieldStatsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                FieldStatsUrlParams::None => Url::from("/_field_stats"),
                FieldStatsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(14usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_field_stats");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct FieldStatsRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> FieldStatsRequest<'a, B> {
        pub fn new(body: B) -> Self {
            FieldStatsRequest {
                url: FieldStatsUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            FieldStatsRequest {
                url: FieldStatsUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for FieldStatsRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatThreadPoolUrlParams<'a> {
        None,
        ThreadPoolPatterns(ThreadPoolPatterns<'a>),
    }
    impl<'a> CatThreadPoolUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatThreadPoolUrlParams::None => Url::from("/_cat/thread_pool"),
                CatThreadPoolUrlParams::ThreadPoolPatterns(ref thread_pool_patterns) => {
                    let mut url = String::with_capacity(18usize + thread_pool_patterns.len());
                    url.push_str("/_cat/thread_pool/");
                    url.push_str(thread_pool_patterns.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatThreadPoolRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatThreadPoolRequest<'a> {
        pub fn new() -> Self {
            CatThreadPoolRequest {
                url: CatThreadPoolUrlParams::None.url(),
            }
        }
        pub fn for_thread_pool_patterns<IThreadPoolPatterns>(thread_pool_patterns: IThreadPoolPatterns) -> Self
        where
            IThreadPoolPatterns: Into<ThreadPoolPatterns<'a>>,
        {
            CatThreadPoolRequest {
                url: CatThreadPoolUrlParams::ThreadPoolPatterns(thread_pool_patterns.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatThreadPoolRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotDeleteUrlParams<'a> {
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotDeleteUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SnapshotDeleteUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url = String::with_capacity(12usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SnapshotDeleteRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> SnapshotDeleteRequest<'a> {
        pub fn for_repository_snapshot<IRepository, ISnapshot>(repository: IRepository, snapshot: ISnapshot) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotDeleteRequest {
                url: SnapshotDeleteUrlParams::RepositorySnapshot(repository.into(), snapshot.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for SnapshotDeleteRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetSettingsUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexName(Index<'a>, Name<'a>),
        Name(Name<'a>),
    }
    impl<'a> IndicesGetSettingsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesGetSettingsUrlParams::None => Url::from("/_settings"),
                IndicesGetSettingsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(11usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_settings");
                    Url::from(url)
                }
                IndicesGetSettingsUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(12usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_settings/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
                IndicesGetSettingsUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_settings/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesGetSettingsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesGetSettingsRequest<'a> {
        pub fn new() -> Self {
            IndicesGetSettingsRequest {
                url: IndicesGetSettingsUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetSettingsRequest {
                url: IndicesGetSettingsUrlParams::Index(index.into()).url(),
            }
        }
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesGetSettingsRequest {
                url: IndicesGetSettingsUrlParams::IndexName(index.into(), name.into()).url(),
            }
        }
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesGetSettingsRequest {
                url: IndicesGetSettingsUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesGetSettingsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CreateUrlParams<'a> {
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> CreateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CreateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_create");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CreateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> CreateRequest<'a, B> {
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            CreateRequest {
                url: CreateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for CreateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotDeleteRepositoryUrlParams<'a> {
        Repository(Repository<'a>),
    }
    impl<'a> SnapshotDeleteRepositoryUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SnapshotDeleteRepositoryUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(11usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SnapshotDeleteRepositoryRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> SnapshotDeleteRepositoryRequest<'a> {
        pub fn for_repository<IRepository>(repository: IRepository) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotDeleteRepositoryRequest {
                url: SnapshotDeleteRepositoryUrlParams::Repository(repository.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for SnapshotDeleteRepositoryRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterAllocationExplainUrlParams {
        None,
    }
    impl ClusterAllocationExplainUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                ClusterAllocationExplainUrlParams::None => Url::from("/_cluster/allocation/explain"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ClusterAllocationExplainRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> ClusterAllocationExplainRequest<'a, B> {
        pub fn new(body: B) -> Self {
            ClusterAllocationExplainRequest {
                url: ClusterAllocationExplainUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for ClusterAllocationExplainRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesPutTemplateUrlParams<'a> {
        Name(Name<'a>),
    }
    impl<'a> IndicesPutTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesPutTemplateUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_template/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesPutTemplateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesPutTemplateRequest<'a, B> {
        pub fn for_name<IName>(name: IName, body: B) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesPutTemplateRequest {
                url: IndicesPutTemplateUrlParams::Name(name.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesPutTemplateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetTemplateUrlParams<'a> {
        None,
        Name(Name<'a>),
    }
    impl<'a> IndicesGetTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesGetTemplateUrlParams::None => Url::from("/_template"),
                IndicesGetTemplateUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_template/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesGetTemplateRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesGetTemplateRequest<'a> {
        pub fn new() -> Self {
            IndicesGetTemplateRequest {
                url: IndicesGetTemplateUrlParams::None.url(),
            }
        }
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesGetTemplateRequest {
                url: IndicesGetTemplateUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesGetTemplateRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterStateUrlParams<'a> {
        None,
        Metric(Metric<'a>),
        MetricIndex(Metric<'a>, Index<'a>),
    }
    impl<'a> ClusterStateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                ClusterStateUrlParams::None => Url::from("/_cluster/state"),
                ClusterStateUrlParams::Metric(ref metric) => {
                    let mut url = String::with_capacity(16usize + metric.len());
                    url.push_str("/_cluster/state/");
                    url.push_str(metric.as_ref());
                    Url::from(url)
                }
                ClusterStateUrlParams::MetricIndex(ref metric, ref index) => {
                    let mut url = String::with_capacity(17usize + metric.len() + index.len());
                    url.push_str("/_cluster/state/");
                    url.push_str(metric.as_ref());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ClusterStateRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> ClusterStateRequest<'a> {
        pub fn new() -> Self {
            ClusterStateRequest {
                url: ClusterStateUrlParams::None.url(),
            }
        }
        pub fn for_metric<IMetric>(metric: IMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
        {
            ClusterStateRequest {
                url: ClusterStateUrlParams::Metric(metric.into()).url(),
            }
        }
        pub fn for_metric_index<IMetric, IIndex>(metric: IMetric, index: IIndex) -> Self
        where
            IMetric: Into<Metric<'a>>,
            IIndex: Into<Index<'a>>,
        {
            ClusterStateRequest {
                url: ClusterStateUrlParams::MetricIndex(metric.into(), index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for ClusterStateRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum MsearchTemplateUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> MsearchTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                MsearchTemplateUrlParams::None => Url::from("/_msearch/template"),
                MsearchTemplateUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(19usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_msearch/template");
                    Url::from(url)
                }
                MsearchTemplateUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(20usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_msearch/template");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct MsearchTemplateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> MsearchTemplateRequest<'a, B> {
        pub fn new(body: B) -> Self {
            MsearchTemplateRequest {
                url: MsearchTemplateUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            MsearchTemplateRequest {
                url: MsearchTemplateUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            MsearchTemplateRequest {
                url: MsearchTemplateUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for MsearchTemplateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum BulkUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> BulkUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                BulkUrlParams::None => Url::from("/_bulk"),
                BulkUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(7usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_bulk");
                    Url::from(url)
                }
                BulkUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(8usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_bulk");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct BulkRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> BulkRequest<'a, B> {
        pub fn new(body: B) -> Self {
            BulkRequest {
                url: BulkUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            BulkRequest {
                url: BulkUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            BulkRequest {
                url: BulkUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for BulkRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ExplainUrlParams<'a> {
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> ExplainUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                ExplainUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(12usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_explain");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ExplainRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> ExplainRequest<'a, B> {
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            ExplainRequest {
                url: ExplainUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for ExplainRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SuggestUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> SuggestUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SuggestUrlParams::None => Url::from("/_suggest"),
                SuggestUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_suggest");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SuggestRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> SuggestRequest<'a, B> {
        pub fn new(body: B) -> Self {
            SuggestRequest {
                url: SuggestUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            SuggestRequest {
                url: SuggestUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for SuggestRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotGetRepositoryUrlParams<'a> {
        None,
        Repository(Repository<'a>),
    }
    impl<'a> SnapshotGetRepositoryUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SnapshotGetRepositoryUrlParams::None => Url::from("/_snapshot"),
                SnapshotGetRepositoryUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(11usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SnapshotGetRepositoryRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> SnapshotGetRepositoryRequest<'a> {
        pub fn new() -> Self {
            SnapshotGetRepositoryRequest {
                url: SnapshotGetRepositoryUrlParams::None.url(),
            }
        }
        pub fn for_repository<IRepository>(repository: IRepository) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotGetRepositoryRequest {
                url: SnapshotGetRepositoryUrlParams::Repository(repository.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for SnapshotGetRepositoryRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum RenderSearchTemplateUrlParams<'a> {
        None,
        Id(Id<'a>),
    }
    impl<'a> RenderSearchTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                RenderSearchTemplateUrlParams::None => Url::from("/_render/template"),
                RenderSearchTemplateUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_render/template/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct RenderSearchTemplateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> RenderSearchTemplateRequest<'a, B> {
        pub fn new(body: B) -> Self {
            RenderSearchTemplateRequest {
                url: RenderSearchTemplateUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_id<IId>(id: IId, body: B) -> Self
        where
            IId: Into<Id<'a>>,
        {
            RenderSearchTemplateRequest {
                url: RenderSearchTemplateUrlParams::Id(id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for RenderSearchTemplateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesStatsUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexMetric(Index<'a>, Metric<'a>),
        Metric(Metric<'a>),
    }
    impl<'a> IndicesStatsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesStatsUrlParams::None => Url::from("/_stats"),
                IndicesStatsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_stats");
                    Url::from(url)
                }
                IndicesStatsUrlParams::IndexMetric(ref index, ref metric) => {
                    let mut url = String::with_capacity(9usize + index.len() + metric.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_stats/");
                    url.push_str(metric.as_ref());
                    Url::from(url)
                }
                IndicesStatsUrlParams::Metric(ref metric) => {
                    let mut url = String::with_capacity(8usize + metric.len());
                    url.push_str("/_stats/");
                    url.push_str(metric.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesStatsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesStatsRequest<'a> {
        pub fn new() -> Self {
            IndicesStatsRequest {
                url: IndicesStatsUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesStatsRequest {
                url: IndicesStatsUrlParams::Index(index.into()).url(),
            }
        }
        pub fn for_index_metric<IIndex, IMetric>(index: IIndex, metric: IMetric) -> Self
        where
            IIndex: Into<Index<'a>>,
            IMetric: Into<Metric<'a>>,
        {
            IndicesStatsRequest {
                url: IndicesStatsUrlParams::IndexMetric(index.into(), metric.into()).url(),
            }
        }
        pub fn for_metric<IMetric>(metric: IMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
        {
            IndicesStatsRequest {
                url: IndicesStatsUrlParams::Metric(metric.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesStatsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatRepositoriesUrlParams {
        None,
    }
    impl CatRepositoriesUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                CatRepositoriesUrlParams::None => Url::from("/_cat/repositories"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatRepositoriesRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatRepositoriesRequest<'a> {
        pub fn new() -> Self {
            CatRepositoriesRequest {
                url: CatRepositoriesUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatRepositoriesRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesForcemergeUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesForcemergeUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesForcemergeUrlParams::None => Url::from("/_forcemerge"),
                IndicesForcemergeUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(13usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_forcemerge");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesForcemergeRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesForcemergeRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesForcemergeRequest {
                url: IndicesForcemergeUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesForcemergeRequest {
                url: IndicesForcemergeUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesForcemergeRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum PingUrlParams {
        None,
    }
    impl PingUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                PingUrlParams::None => Url::from("/"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct PingRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> PingRequest<'a> {
        pub fn new() -> Self {
            PingRequest {
                url: PingUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for PingRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum PingHeadUrlParams {
        None,
    }
    impl PingHeadUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                PingHeadUrlParams::None => Url::from("/"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct PingHeadRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> PingHeadRequest<'a> {
        pub fn new() -> Self {
            PingHeadRequest {
                url: PingHeadUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for PingHeadRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Head,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum TasksGetUrlParams<'a> {
        TaskId(TaskId<'a>),
    }
    impl<'a> TasksGetUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                TasksGetUrlParams::TaskId(ref task_id) => {
                    let mut url = String::with_capacity(8usize + task_id.len());
                    url.push_str("/_tasks/");
                    url.push_str(task_id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct TasksGetRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> TasksGetRequest<'a> {
        pub fn for_task_id<ITaskId>(task_id: ITaskId) -> Self
        where
            ITaskId: Into<TaskId<'a>>,
        {
            TasksGetRequest {
                url: TasksGetUrlParams::TaskId(task_id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for TasksGetRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesExistsUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesExistsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesExistsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(1usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesExistsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesExistsRequest<'a> {
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesExistsRequest {
                url: IndicesExistsUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesExistsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Head,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesFlushSyncedUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesFlushSyncedUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesFlushSyncedUrlParams::None => Url::from("/_flush/synced"),
                IndicesFlushSyncedUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_flush/synced");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesFlushSyncedRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesFlushSyncedRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesFlushSyncedRequest {
                url: IndicesFlushSyncedUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesFlushSyncedRequest {
                url: IndicesFlushSyncedUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesFlushSyncedRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum MsearchUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> MsearchUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                MsearchUrlParams::None => Url::from("/_msearch"),
                MsearchUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_msearch");
                    Url::from(url)
                }
                MsearchUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(11usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_msearch");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct MsearchRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> MsearchRequest<'a, B> {
        pub fn new(body: B) -> Self {
            MsearchRequest {
                url: MsearchUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            MsearchRequest {
                url: MsearchUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            MsearchRequest {
                url: MsearchUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for MsearchRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum InfoUrlParams {
        None,
    }
    impl InfoUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                InfoUrlParams::None => Url::from("/"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct InfoRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> InfoRequest<'a> {
        pub fn new() -> Self {
            InfoRequest {
                url: InfoUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for InfoRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SearchTemplateUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> SearchTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SearchTemplateUrlParams::None => Url::from("/_search/template"),
                SearchTemplateUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(18usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_search/template");
                    Url::from(url)
                }
                SearchTemplateUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(19usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_search/template");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SearchTemplateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> SearchTemplateRequest<'a, B> {
        pub fn new(body: B) -> Self {
            SearchTemplateRequest {
                url: SearchTemplateUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            SearchTemplateRequest {
                url: SearchTemplateUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            SearchTemplateRequest {
                url: SearchTemplateUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for SearchTemplateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesDeleteUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesDeleteUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesDeleteUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(1usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesDeleteRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesDeleteRequest<'a> {
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesDeleteRequest {
                url: IndicesDeleteUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesDeleteRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum DeleteByQueryUrlParams<'a> {
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> DeleteByQueryUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                DeleteByQueryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(18usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_delete_by_query");
                    Url::from(url)
                }
                DeleteByQueryUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(19usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_delete_by_query");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct DeleteByQueryRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> DeleteByQueryRequest<'a, B> {
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            DeleteByQueryRequest {
                url: DeleteByQueryUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            DeleteByQueryRequest {
                url: DeleteByQueryUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for DeleteByQueryRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum DeleteTemplateUrlParams<'a> {
        Id(Id<'a>),
    }
    impl<'a> DeleteTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                DeleteTemplateUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_search/template/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct DeleteTemplateRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> DeleteTemplateRequest<'a> {
        pub fn for_id<IId>(id: IId) -> Self
        where
            IId: Into<Id<'a>>,
        {
            DeleteTemplateRequest {
                url: DeleteTemplateUrlParams::Id(id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for DeleteTemplateRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesCreateUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesCreateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesCreateUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(1usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesCreateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesCreateRequest<'a, B> {
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesCreateRequest {
                url: IndicesCreateUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesCreateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Put,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum PercolateUrlParams<'a> {
        IndexType(Index<'a>, Type<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> PercolateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                PercolateUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(13usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_percolate");
                    Url::from(url)
                }
                PercolateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(14usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_percolate");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct PercolateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> PercolateRequest<'a, B> {
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            PercolateRequest {
                url: PercolateUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            PercolateRequest {
                url: PercolateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for PercolateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SimpleSearchUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> SimpleSearchUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SimpleSearchUrlParams::None => Url::from("/_search"),
                SimpleSearchUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(9usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_search");
                    Url::from(url)
                }
                SimpleSearchUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(10usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_search");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SimpleSearchRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> SimpleSearchRequest<'a> {
        pub fn new() -> Self {
            SimpleSearchRequest {
                url: SimpleSearchUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            SimpleSearchRequest {
                url: SimpleSearchUrlParams::Index(index.into()).url(),
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            SimpleSearchRequest {
                url: SimpleSearchUrlParams::IndexType(index.into(), ty.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for SimpleSearchRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SearchUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> SearchUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SearchUrlParams::None => Url::from("/_search"),
                SearchUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(9usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_search");
                    Url::from(url)
                }
                SearchUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(10usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_search");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SearchRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> SearchRequest<'a, B> {
        pub fn new(body: B) -> Self {
            SearchRequest {
                url: SearchUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            SearchRequest {
                url: SearchUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            SearchRequest {
                url: SearchUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for SearchRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatNodeattrsUrlParams {
        None,
    }
    impl CatNodeattrsUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                CatNodeattrsUrlParams::None => Url::from("/_cat/nodeattrs"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatNodeattrsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatNodeattrsRequest<'a> {
        pub fn new() -> Self {
            CatNodeattrsRequest {
                url: CatNodeattrsUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatNodeattrsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotVerifyRepositoryUrlParams<'a> {
        Repository(Repository<'a>),
    }
    impl<'a> SnapshotVerifyRepositoryUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SnapshotVerifyRepositoryUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(19usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/_verify");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SnapshotVerifyRepositoryRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> SnapshotVerifyRepositoryRequest<'a, B> {
        pub fn for_repository<IRepository>(repository: IRepository, body: B) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotVerifyRepositoryRequest {
                url: SnapshotVerifyRepositoryUrlParams::Repository(repository.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for SnapshotVerifyRepositoryRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CountUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> CountUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CountUrlParams::None => Url::from("/_count"),
                CountUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_count");
                    Url::from(url)
                }
                CountUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(9usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_count");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CountRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> CountRequest<'a, B> {
        pub fn new(body: B) -> Self {
            CountRequest {
                url: CountUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CountRequest {
                url: CountUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            CountRequest {
                url: CountUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for CountRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatAllocationUrlParams<'a> {
        None,
        NodeId(NodeId<'a>),
    }
    impl<'a> CatAllocationUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatAllocationUrlParams::None => Url::from("/_cat/allocation"),
                CatAllocationUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(17usize + node_id.len());
                    url.push_str("/_cat/allocation/");
                    url.push_str(node_id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatAllocationRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatAllocationRequest<'a> {
        pub fn new() -> Self {
            CatAllocationRequest {
                url: CatAllocationUrlParams::None.url(),
            }
        }
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            CatAllocationRequest {
                url: CatAllocationUrlParams::NodeId(node_id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatAllocationRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesFlushUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesFlushUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesFlushUrlParams::None => Url::from("/_flush"),
                IndicesFlushUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_flush");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesFlushRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesFlushRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesFlushRequest {
                url: IndicesFlushUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesFlushRequest {
                url: IndicesFlushUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesFlushRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesRefreshUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesRefreshUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesRefreshUrlParams::None => Url::from("/_refresh"),
                IndicesRefreshUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_refresh");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesRefreshRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesRefreshRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesRefreshRequest {
                url: IndicesRefreshUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesRefreshRequest {
                url: IndicesRefreshUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesRefreshRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatHelpUrlParams {
        None,
    }
    impl CatHelpUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                CatHelpUrlParams::None => Url::from("/_cat"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatHelpRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatHelpRequest<'a> {
        pub fn new() -> Self {
            CatHelpRequest {
                url: CatHelpUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatHelpRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SearchShardsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> SearchShardsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SearchShardsUrlParams::None => Url::from("/_search_shards"),
                SearchShardsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(16usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_search_shards");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SearchShardsRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> SearchShardsRequest<'a, B> {
        pub fn new(body: B) -> Self {
            SearchShardsRequest {
                url: SearchShardsUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            SearchShardsRequest {
                url: SearchShardsUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for SearchShardsRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterHealthUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> ClusterHealthUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                ClusterHealthUrlParams::None => Url::from("/_cluster/health"),
                ClusterHealthUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(17usize + index.len());
                    url.push_str("/_cluster/health/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ClusterHealthRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> ClusterHealthRequest<'a> {
        pub fn new() -> Self {
            ClusterHealthRequest {
                url: ClusterHealthUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            ClusterHealthRequest {
                url: ClusterHealthUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for ClusterHealthRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesExistsAliasUrlParams<'a> {
        Index(Index<'a>),
        IndexName(Index<'a>, Name<'a>),
        Name(Name<'a>),
    }
    impl<'a> IndicesExistsAliasUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesExistsAliasUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_alias");
                    Url::from(url)
                }
                IndicesExistsAliasUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(9usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_alias/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
                IndicesExistsAliasUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(8usize + name.len());
                    url.push_str("/_alias/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesExistsAliasRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesExistsAliasRequest<'a> {
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesExistsAliasRequest {
                url: IndicesExistsAliasUrlParams::Index(index.into()).url(),
            }
        }
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesExistsAliasRequest {
                url: IndicesExistsAliasUrlParams::IndexName(index.into(), name.into()).url(),
            }
        }
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesExistsAliasRequest {
                url: IndicesExistsAliasUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesExistsAliasRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Head,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetFieldMappingUrlParams<'a> {
        Fields(Fields<'a>),
        IndexFields(Index<'a>, Fields<'a>),
        IndexTypeFields(Index<'a>, Type<'a>, Fields<'a>),
        TypeFields(Type<'a>, Fields<'a>),
    }
    impl<'a> IndicesGetFieldMappingUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesGetFieldMappingUrlParams::Fields(ref fields) => {
                    let mut url = String::with_capacity(16usize + fields.len());
                    url.push_str("/_mapping/field/");
                    url.push_str(fields.as_ref());
                    Url::from(url)
                }
                IndicesGetFieldMappingUrlParams::IndexFields(ref index, ref fields) => {
                    let mut url = String::with_capacity(17usize + index.len() + fields.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping/field/");
                    url.push_str(fields.as_ref());
                    Url::from(url)
                }
                IndicesGetFieldMappingUrlParams::IndexTypeFields(ref index, ref ty, ref fields) => {
                    let mut url = String::with_capacity(18usize + index.len() + ty.len() + fields.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    url.push_str("/field/");
                    url.push_str(fields.as_ref());
                    Url::from(url)
                }
                IndicesGetFieldMappingUrlParams::TypeFields(ref ty, ref fields) => {
                    let mut url = String::with_capacity(17usize + ty.len() + fields.len());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    url.push_str("/field/");
                    url.push_str(fields.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesGetFieldMappingRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesGetFieldMappingRequest<'a> {
        pub fn for_fields<IFields>(fields: IFields) -> Self
        where
            IFields: Into<Fields<'a>>,
        {
            IndicesGetFieldMappingRequest {
                url: IndicesGetFieldMappingUrlParams::Fields(fields.into()).url(),
            }
        }
        pub fn for_index_fields<IIndex, IFields>(index: IIndex, fields: IFields) -> Self
        where
            IIndex: Into<Index<'a>>,
            IFields: Into<Fields<'a>>,
        {
            IndicesGetFieldMappingRequest {
                url: IndicesGetFieldMappingUrlParams::IndexFields(index.into(), fields.into()).url(),
            }
        }
        pub fn for_index_ty_fields<IIndex, IType, IFields>(index: IIndex, ty: IType, fields: IFields) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IFields: Into<Fields<'a>>,
        {
            IndicesGetFieldMappingRequest {
                url: IndicesGetFieldMappingUrlParams::IndexTypeFields(index.into(), ty.into(), fields.into()).url(),
            }
        }
        pub fn for_ty_fields<IType, IFields>(ty: IType, fields: IFields) -> Self
        where
            IType: Into<Type<'a>>,
            IFields: Into<Fields<'a>>,
        {
            IndicesGetFieldMappingRequest {
                url: IndicesGetFieldMappingUrlParams::TypeFields(ty.into(), fields.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesGetFieldMappingRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IngestPutPipelineUrlParams<'a> {
        Id(Id<'a>),
    }
    impl<'a> IngestPutPipelineUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IngestPutPipelineUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_ingest/pipeline/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IngestPutPipelineRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IngestPutPipelineRequest<'a, B> {
        pub fn for_id<IId>(id: IId, body: B) -> Self
        where
            IId: Into<Id<'a>>,
        {
            IngestPutPipelineRequest {
                url: IngestPutPipelineUrlParams::Id(id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IngestPutPipelineRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Put,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterPendingTasksUrlParams {
        None,
    }
    impl ClusterPendingTasksUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                ClusterPendingTasksUrlParams::None => Url::from("/_cluster/pending_tasks"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ClusterPendingTasksRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> ClusterPendingTasksRequest<'a> {
        pub fn new() -> Self {
            ClusterPendingTasksRequest {
                url: ClusterPendingTasksUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for ClusterPendingTasksRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IngestSimulateUrlParams<'a> {
        None,
        Id(Id<'a>),
    }
    impl<'a> IngestSimulateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IngestSimulateUrlParams::None => Url::from("/_ingest/pipeline/_simulate"),
                IngestSimulateUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(28usize + id.len());
                    url.push_str("/_ingest/pipeline/");
                    url.push_str(id.as_ref());
                    url.push_str("/_simulate");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IngestSimulateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IngestSimulateRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IngestSimulateRequest {
                url: IngestSimulateUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_id<IId>(id: IId, body: B) -> Self
        where
            IId: Into<Id<'a>>,
        {
            IngestSimulateRequest {
                url: IngestSimulateUrlParams::Id(id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IngestSimulateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetAliasUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexName(Index<'a>, Name<'a>),
        Name(Name<'a>),
    }
    impl<'a> IndicesGetAliasUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesGetAliasUrlParams::None => Url::from("/_alias"),
                IndicesGetAliasUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(8usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_alias");
                    Url::from(url)
                }
                IndicesGetAliasUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(9usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_alias/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
                IndicesGetAliasUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(8usize + name.len());
                    url.push_str("/_alias/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesGetAliasRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesGetAliasRequest<'a> {
        pub fn new() -> Self {
            IndicesGetAliasRequest {
                url: IndicesGetAliasUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetAliasRequest {
                url: IndicesGetAliasUrlParams::Index(index.into()).url(),
            }
        }
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesGetAliasRequest {
                url: IndicesGetAliasUrlParams::IndexName(index.into(), name.into()).url(),
            }
        }
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesGetAliasRequest {
                url: IndicesGetAliasUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesGetAliasRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum GetScriptUrlParams<'a> {
        LangId(Lang<'a>, Id<'a>),
    }
    impl<'a> GetScriptUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                GetScriptUrlParams::LangId(ref lang, ref id) => {
                    let mut url = String::with_capacity(11usize + lang.len() + id.len());
                    url.push_str("/_scripts/");
                    url.push_str(lang.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct GetScriptRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> GetScriptRequest<'a> {
        pub fn for_lang_id<ILang, IId>(lang: ILang, id: IId) -> Self
        where
            ILang: Into<Lang<'a>>,
            IId: Into<Id<'a>>,
        {
            GetScriptRequest {
                url: GetScriptUrlParams::LangId(lang.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for GetScriptRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesRecoveryUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesRecoveryUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesRecoveryUrlParams::None => Url::from("/_recovery"),
                IndicesRecoveryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(11usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_recovery");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesRecoveryRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesRecoveryRequest<'a> {
        pub fn new() -> Self {
            IndicesRecoveryRequest {
                url: IndicesRecoveryUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesRecoveryRequest {
                url: IndicesRecoveryUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesRecoveryRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IngestDeletePipelineUrlParams<'a> {
        Id(Id<'a>),
    }
    impl<'a> IngestDeletePipelineUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IngestDeletePipelineUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_ingest/pipeline/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IngestDeletePipelineRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IngestDeletePipelineRequest<'a> {
        pub fn for_id<IId>(id: IId) -> Self
        where
            IId: Into<Id<'a>>,
        {
            IngestDeletePipelineRequest {
                url: IngestDeletePipelineUrlParams::Id(id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IngestDeletePipelineRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum TasksCancelUrlParams<'a> {
        None,
        TaskId(TaskId<'a>),
    }
    impl<'a> TasksCancelUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                TasksCancelUrlParams::None => Url::from("/_tasks/_cancel"),
                TasksCancelUrlParams::TaskId(ref task_id) => {
                    let mut url = String::with_capacity(16usize + task_id.len());
                    url.push_str("/_tasks/");
                    url.push_str(task_id.as_ref());
                    url.push_str("/_cancel");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct TasksCancelRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> TasksCancelRequest<'a, B> {
        pub fn new(body: B) -> Self {
            TasksCancelRequest {
                url: TasksCancelUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_task_id<ITaskId>(task_id: ITaskId, body: B) -> Self
        where
            ITaskId: Into<TaskId<'a>>,
        {
            TasksCancelRequest {
                url: TasksCancelUrlParams::TaskId(task_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for TasksCancelRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesClearCacheUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesClearCacheUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesClearCacheUrlParams::None => Url::from("/_cache/clear"),
                IndicesClearCacheUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(14usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_cache/clear");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesClearCacheRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesClearCacheRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesClearCacheRequest {
                url: IndicesClearCacheUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesClearCacheRequest {
                url: IndicesClearCacheUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesClearCacheRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum DeleteUrlParams<'a> {
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> DeleteUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                DeleteUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct DeleteRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> DeleteRequest<'a> {
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            DeleteRequest {
                url: DeleteUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for DeleteRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesPutMappingUrlParams<'a> {
        IndexType(Index<'a>, Type<'a>),
        Type(Type<'a>),
    }
    impl<'a> IndicesPutMappingUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesPutMappingUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(12usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mappings/");
                    url.push_str(ty.as_ref());
                    Url::from(url)
                }
                IndicesPutMappingUrlParams::Type(ref ty) => {
                    let mut url = String::with_capacity(11usize + ty.len());
                    url.push_str("/_mappings/");
                    url.push_str(ty.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesPutMappingRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesPutMappingRequest<'a, B> {
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndicesPutMappingRequest {
                url: IndicesPutMappingUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
        pub fn for_ty<IType>(ty: IType, body: B) -> Self
        where
            IType: Into<Type<'a>>,
        {
            IndicesPutMappingRequest {
                url: IndicesPutMappingUrlParams::Type(ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesPutMappingRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatAliasesUrlParams<'a> {
        None,
        Name(Name<'a>),
    }
    impl<'a> CatAliasesUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatAliasesUrlParams::None => Url::from("/_cat/aliases"),
                CatAliasesUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(14usize + name.len());
                    url.push_str("/_cat/aliases/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatAliasesRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatAliasesRequest<'a> {
        pub fn new() -> Self {
            CatAliasesRequest {
                url: CatAliasesUrlParams::None.url(),
            }
        }
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            CatAliasesRequest {
                url: CatAliasesUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatAliasesRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterStatsUrlParams<'a> {
        None,
        NodeId(NodeId<'a>),
    }
    impl<'a> ClusterStatsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                ClusterStatsUrlParams::None => Url::from("/_cluster/stats"),
                ClusterStatsUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(22usize + node_id.len());
                    url.push_str("/_cluster/stats/nodes/");
                    url.push_str(node_id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ClusterStatsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> ClusterStatsRequest<'a> {
        pub fn new() -> Self {
            ClusterStatsRequest {
                url: ClusterStatsUrlParams::None.url(),
            }
        }
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            ClusterStatsRequest {
                url: ClusterStatsUrlParams::NodeId(node_id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for ClusterStatsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesValidateQueryUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> IndicesValidateQueryUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesValidateQueryUrlParams::None => Url::from("/_validate/query"),
                IndicesValidateQueryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(17usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_validate/query");
                    Url::from(url)
                }
                IndicesValidateQueryUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(18usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_validate/query");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesValidateQueryRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesValidateQueryRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesValidateQueryRequest {
                url: IndicesValidateQueryUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesValidateQueryRequest {
                url: IndicesValidateQueryUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndicesValidateQueryRequest {
                url: IndicesValidateQueryUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesValidateQueryRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatPendingTasksUrlParams {
        None,
    }
    impl CatPendingTasksUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                CatPendingTasksUrlParams::None => Url::from("/_cat/pending_tasks"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatPendingTasksRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatPendingTasksRequest<'a> {
        pub fn new() -> Self {
            CatPendingTasksRequest {
                url: CatPendingTasksUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatPendingTasksRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClearScrollUrlParams<'a> {
        None,
        ScrollId(ScrollId<'a>),
    }
    impl<'a> ClearScrollUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                ClearScrollUrlParams::None => Url::from("/_search/scroll"),
                ClearScrollUrlParams::ScrollId(ref scroll_id) => {
                    let mut url = String::with_capacity(16usize + scroll_id.len());
                    url.push_str("/_search/scroll/");
                    url.push_str(scroll_id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ClearScrollRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> ClearScrollRequest<'a, B> {
        pub fn new(body: B) -> Self {
            ClearScrollRequest {
                url: ClearScrollUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_scroll_id<IScrollId>(scroll_id: IScrollId, body: B) -> Self
        where
            IScrollId: Into<ScrollId<'a>>,
        {
            ClearScrollRequest {
                url: ClearScrollUrlParams::ScrollId(scroll_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for ClearScrollRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatShardsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatShardsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatShardsUrlParams::None => Url::from("/_cat/shards"),
                CatShardsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(13usize + index.len());
                    url.push_str("/_cat/shards/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatShardsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatShardsRequest<'a> {
        pub fn new() -> Self {
            CatShardsRequest {
                url: CatShardsUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatShardsRequest {
                url: CatShardsUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatShardsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesShardStoresUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesShardStoresUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesShardStoresUrlParams::None => Url::from("/_shard_stores"),
                IndicesShardStoresUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_shard_stores");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesShardStoresRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesShardStoresRequest<'a> {
        pub fn new() -> Self {
            IndicesShardStoresRequest {
                url: IndicesShardStoresUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesShardStoresRequest {
                url: IndicesShardStoresUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesShardStoresRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesUpdateAliasesUrlParams {
        None,
    }
    impl IndicesUpdateAliasesUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                IndicesUpdateAliasesUrlParams::None => Url::from("/_aliases"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesUpdateAliasesRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesUpdateAliasesRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesUpdateAliasesRequest {
                url: IndicesUpdateAliasesUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesUpdateAliasesRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatSegmentsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatSegmentsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatSegmentsUrlParams::None => Url::from("/_cat/segments"),
                CatSegmentsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/_cat/segments/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatSegmentsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatSegmentsRequest<'a> {
        pub fn new() -> Self {
            CatSegmentsRequest {
                url: CatSegmentsUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatSegmentsRequest {
                url: CatSegmentsUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatSegmentsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum MpercolateUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> MpercolateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                MpercolateUrlParams::None => Url::from("/_mpercolate"),
                MpercolateUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(13usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mpercolate");
                    Url::from(url)
                }
                MpercolateUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(14usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_mpercolate");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct MpercolateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> MpercolateRequest<'a, B> {
        pub fn new(body: B) -> Self {
            MpercolateRequest {
                url: MpercolateUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            MpercolateRequest {
                url: MpercolateUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            MpercolateRequest {
                url: MpercolateUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for MpercolateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesOpenUrlParams<'a> {
        Index(Index<'a>),
    }
    impl<'a> IndicesOpenUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesOpenUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(7usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_open");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesOpenRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesOpenRequest<'a, B> {
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesOpenRequest {
                url: IndicesOpenUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesOpenRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum GetUrlParams<'a> {
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> GetUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                GetUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct GetRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> GetRequest<'a> {
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            GetRequest {
                url: GetUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for GetRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum UpdateByQueryUrlParams<'a> {
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> UpdateByQueryUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                UpdateByQueryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(18usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_update_by_query");
                    Url::from(url)
                }
                UpdateByQueryUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(19usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_update_by_query");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct UpdateByQueryRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> UpdateByQueryRequest<'a, B> {
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            UpdateByQueryRequest {
                url: UpdateByQueryUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            UpdateByQueryRequest {
                url: UpdateByQueryUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for UpdateByQueryRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum MtermvectorsUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> MtermvectorsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                MtermvectorsUrlParams::None => Url::from("/_mtermvectors"),
                MtermvectorsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mtermvectors");
                    Url::from(url)
                }
                MtermvectorsUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(16usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_mtermvectors");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct MtermvectorsRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> MtermvectorsRequest<'a, B> {
        pub fn new(body: B) -> Self {
            MtermvectorsRequest {
                url: MtermvectorsUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            MtermvectorsRequest {
                url: MtermvectorsUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            MtermvectorsRequest {
                url: MtermvectorsUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for MtermvectorsRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatRecoveryUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatRecoveryUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatRecoveryUrlParams::None => Url::from("/_cat/recovery"),
                CatRecoveryUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(15usize + index.len());
                    url.push_str("/_cat/recovery/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatRecoveryRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatRecoveryRequest<'a> {
        pub fn new() -> Self {
            CatRecoveryRequest {
                url: CatRecoveryUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatRecoveryRequest {
                url: CatRecoveryUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatRecoveryRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotRestoreUrlParams<'a> {
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotRestoreUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SnapshotRestoreUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url = String::with_capacity(21usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    url.push_str("/_restore");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SnapshotRestoreRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> SnapshotRestoreRequest<'a, B> {
        pub fn for_repository_snapshot<IRepository, ISnapshot>(repository: IRepository, snapshot: ISnapshot, body: B) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotRestoreRequest {
                url: SnapshotRestoreUrlParams::RepositorySnapshot(repository.into(), snapshot.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for SnapshotRestoreRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ReindexUrlParams {
        None,
    }
    impl ReindexUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                ReindexUrlParams::None => Url::from("/_reindex"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ReindexRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> ReindexRequest<'a, B> {
        pub fn new(body: B) -> Self {
            ReindexRequest {
                url: ReindexUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for ReindexRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatHealthUrlParams {
        None,
    }
    impl CatHealthUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                CatHealthUrlParams::None => Url::from("/_cat/health"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatHealthRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatHealthRequest<'a> {
        pub fn new() -> Self {
            CatHealthRequest {
                url: CatHealthUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatHealthRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatCountUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatCountUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatCountUrlParams::None => Url::from("/_cat/count"),
                CatCountUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(12usize + index.len());
                    url.push_str("/_cat/count/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatCountRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatCountRequest<'a> {
        pub fn new() -> Self {
            CatCountRequest {
                url: CatCountUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatCountRequest {
                url: CatCountUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatCountRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatSnapshotsUrlParams<'a> {
        None,
        Repository(Repository<'a>),
    }
    impl<'a> CatSnapshotsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatSnapshotsUrlParams::None => Url::from("/_cat/snapshots"),
                CatSnapshotsUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(16usize + repository.len());
                    url.push_str("/_cat/snapshots/");
                    url.push_str(repository.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatSnapshotsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatSnapshotsRequest<'a> {
        pub fn new() -> Self {
            CatSnapshotsRequest {
                url: CatSnapshotsUrlParams::None.url(),
            }
        }
        pub fn for_repository<IRepository>(repository: IRepository) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            CatSnapshotsRequest {
                url: CatSnapshotsUrlParams::Repository(repository.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatSnapshotsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetMappingUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
        Type(Type<'a>),
    }
    impl<'a> IndicesGetMappingUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesGetMappingUrlParams::None => Url::from("/_mapping"),
                IndicesGetMappingUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping");
                    Url::from(url)
                }
                IndicesGetMappingUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(11usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    Url::from(url)
                }
                IndicesGetMappingUrlParams::Type(ref ty) => {
                    let mut url = String::with_capacity(10usize + ty.len());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesGetMappingRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesGetMappingRequest<'a> {
        pub fn new() -> Self {
            IndicesGetMappingRequest {
                url: IndicesGetMappingUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetMappingRequest {
                url: IndicesGetMappingUrlParams::Index(index.into()).url(),
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndicesGetMappingRequest {
                url: IndicesGetMappingUrlParams::IndexType(index.into(), ty.into()).url(),
            }
        }
        pub fn for_ty<IType>(ty: IType) -> Self
        where
            IType: Into<Type<'a>>,
        {
            IndicesGetMappingRequest {
                url: IndicesGetMappingUrlParams::Type(ty.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesGetMappingRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotGetUrlParams<'a> {
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotGetUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SnapshotGetUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url = String::with_capacity(12usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SnapshotGetRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> SnapshotGetRequest<'a> {
        pub fn for_repository_snapshot<IRepository, ISnapshot>(repository: IRepository, snapshot: ISnapshot) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotGetRequest {
                url: SnapshotGetUrlParams::RepositorySnapshot(repository.into(), snapshot.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for SnapshotGetRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatNodesUrlParams {
        None,
    }
    impl CatNodesUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                CatNodesUrlParams::None => Url::from("/_cat/nodes"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatNodesRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatNodesRequest<'a> {
        pub fn new() -> Self {
            CatNodesRequest {
                url: CatNodesUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatNodesRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ExistsUrlParams<'a> {
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> ExistsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                ExistsUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ExistsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> ExistsRequest<'a> {
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            ExistsRequest {
                url: ExistsUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for ExistsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Head,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterRerouteUrlParams {
        None,
    }
    impl ClusterRerouteUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                ClusterRerouteUrlParams::None => Url::from("/_cluster/reroute"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ClusterRerouteRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> ClusterRerouteRequest<'a, B> {
        pub fn new(body: B) -> Self {
            ClusterRerouteRequest {
                url: ClusterRerouteUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for ClusterRerouteRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum NodesHotThreadsUrlParams<'a> {
        None,
        NodeId(NodeId<'a>),
    }
    impl<'a> NodesHotThreadsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                NodesHotThreadsUrlParams::None => Url::from("/_nodes/hot_threads"),
                NodesHotThreadsUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(20usize + node_id.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/hot_threads");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct NodesHotThreadsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> NodesHotThreadsRequest<'a> {
        pub fn new() -> Self {
            NodesHotThreadsRequest {
                url: NodesHotThreadsUrlParams::None.url(),
            }
        }
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            NodesHotThreadsRequest {
                url: NodesHotThreadsUrlParams::NodeId(node_id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for NodesHotThreadsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum NodesStatsUrlParams<'a> {
        None,
        Metric(Metric<'a>),
        MetricIndexMetric(Metric<'a>, IndexMetric<'a>),
        NodeId(NodeId<'a>),
        NodeIdMetric(NodeId<'a>, Metric<'a>),
        NodeIdMetricIndexMetric(NodeId<'a>, Metric<'a>, IndexMetric<'a>),
    }
    impl<'a> NodesStatsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                NodesStatsUrlParams::None => Url::from("/_nodes/stats"),
                NodesStatsUrlParams::Metric(ref metric) => {
                    let mut url = String::with_capacity(14usize + metric.len());
                    url.push_str("/_nodes/stats/");
                    url.push_str(metric.as_ref());
                    Url::from(url)
                }
                NodesStatsUrlParams::MetricIndexMetric(ref metric, ref index_metric) => {
                    let mut url = String::with_capacity(15usize + metric.len() + index_metric.len());
                    url.push_str("/_nodes/stats/");
                    url.push_str(metric.as_ref());
                    url.push_str("/");
                    url.push_str(index_metric.as_ref());
                    Url::from(url)
                }
                NodesStatsUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(14usize + node_id.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/stats");
                    Url::from(url)
                }
                NodesStatsUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                    let mut url = String::with_capacity(15usize + node_id.len() + metric.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/stats/");
                    url.push_str(metric.as_ref());
                    Url::from(url)
                }
                NodesStatsUrlParams::NodeIdMetricIndexMetric(ref node_id, ref metric, ref index_metric) => {
                    let mut url = String::with_capacity(16usize + node_id.len() + metric.len() + index_metric.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/stats/");
                    url.push_str(metric.as_ref());
                    url.push_str("/");
                    url.push_str(index_metric.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct NodesStatsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> NodesStatsRequest<'a> {
        pub fn new() -> Self {
            NodesStatsRequest {
                url: NodesStatsUrlParams::None.url(),
            }
        }
        pub fn for_metric<IMetric>(metric: IMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::Metric(metric.into()).url(),
            }
        }
        pub fn for_metric_index_metric<IMetric, IIndexMetric>(metric: IMetric, index_metric: IIndexMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
            IIndexMetric: Into<IndexMetric<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::MetricIndexMetric(metric.into(), index_metric.into()).url(),
            }
        }
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::NodeId(node_id.into()).url(),
            }
        }
        pub fn for_node_id_metric<INodeId, IMetric>(node_id: INodeId, metric: IMetric) -> Self
        where
            INodeId: Into<NodeId<'a>>,
            IMetric: Into<Metric<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::NodeIdMetric(node_id.into(), metric.into()).url(),
            }
        }
        pub fn for_node_id_metric_index_metric<INodeId, IMetric, IIndexMetric>(node_id: INodeId, metric: IMetric, index_metric: IIndexMetric) -> Self
        where
            INodeId: Into<NodeId<'a>>,
            IMetric: Into<Metric<'a>>,
            IIndexMetric: Into<IndexMetric<'a>>,
        {
            NodesStatsRequest {
                url: NodesStatsUrlParams::NodeIdMetricIndexMetric(node_id.into(), metric.into(), index_metric.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for NodesStatsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IngestGetPipelineUrlParams<'a> {
        None,
        Id(Id<'a>),
    }
    impl<'a> IngestGetPipelineUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IngestGetPipelineUrlParams::None => Url::from("/_ingest/pipeline"),
                IngestGetPipelineUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_ingest/pipeline/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IngestGetPipelineRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IngestGetPipelineRequest<'a> {
        pub fn new() -> Self {
            IngestGetPipelineRequest {
                url: IngestGetPipelineUrlParams::None.url(),
            }
        }
        pub fn for_id<IId>(id: IId) -> Self
        where
            IId: Into<Id<'a>>,
        {
            IngestGetPipelineRequest {
                url: IngestGetPipelineUrlParams::Id(id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IngestGetPipelineRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum PutTemplateUrlParams<'a> {
        Id(Id<'a>),
    }
    impl<'a> PutTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                PutTemplateUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_search/template/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct PutTemplateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> PutTemplateRequest<'a, B> {
        pub fn for_id<IId>(id: IId, body: B) -> Self
        where
            IId: Into<Id<'a>>,
        {
            PutTemplateRequest {
                url: PutTemplateUrlParams::Id(id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for PutTemplateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum GetSourceUrlParams<'a> {
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> GetSourceUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                GetSourceUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_source");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct GetSourceRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> GetSourceRequest<'a> {
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            GetSourceRequest {
                url: GetSourceUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for GetSourceRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotCreateUrlParams<'a> {
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotCreateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SnapshotCreateUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url = String::with_capacity(12usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SnapshotCreateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> SnapshotCreateRequest<'a, B> {
        pub fn for_repository_snapshot<IRepository, ISnapshot>(repository: IRepository, snapshot: ISnapshot, body: B) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotCreateRequest {
                url: SnapshotCreateUrlParams::RepositorySnapshot(repository.into(), snapshot.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for SnapshotCreateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ScrollUrlParams<'a> {
        None,
        ScrollId(ScrollId<'a>),
    }
    impl<'a> ScrollUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                ScrollUrlParams::None => Url::from("/_search/scroll"),
                ScrollUrlParams::ScrollId(ref scroll_id) => {
                    let mut url = String::with_capacity(16usize + scroll_id.len());
                    url.push_str("/_search/scroll/");
                    url.push_str(scroll_id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ScrollRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> ScrollRequest<'a, B> {
        pub fn new(body: B) -> Self {
            ScrollRequest {
                url: ScrollUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_scroll_id<IScrollId>(scroll_id: IScrollId, body: B) -> Self
        where
            IScrollId: Into<ScrollId<'a>>,
        {
            ScrollRequest {
                url: ScrollUrlParams::ScrollId(scroll_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for ScrollRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotStatusUrlParams<'a> {
        None,
        Repository(Repository<'a>),
        RepositorySnapshot(Repository<'a>, Snapshot<'a>),
    }
    impl<'a> SnapshotStatusUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SnapshotStatusUrlParams::None => Url::from("/_snapshot/_status"),
                SnapshotStatusUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(19usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/_status");
                    Url::from(url)
                }
                SnapshotStatusUrlParams::RepositorySnapshot(ref repository, ref snapshot) => {
                    let mut url = String::with_capacity(20usize + repository.len() + snapshot.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    url.push_str("/");
                    url.push_str(snapshot.as_ref());
                    url.push_str("/_status");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SnapshotStatusRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> SnapshotStatusRequest<'a> {
        pub fn new() -> Self {
            SnapshotStatusRequest {
                url: SnapshotStatusUrlParams::None.url(),
            }
        }
        pub fn for_repository<IRepository>(repository: IRepository) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotStatusRequest {
                url: SnapshotStatusUrlParams::Repository(repository.into()).url(),
            }
        }
        pub fn for_repository_snapshot<IRepository, ISnapshot>(repository: IRepository, snapshot: ISnapshot) -> Self
        where
            IRepository: Into<Repository<'a>>,
            ISnapshot: Into<Snapshot<'a>>,
        {
            SnapshotStatusRequest {
                url: SnapshotStatusUrlParams::RepositorySnapshot(repository.into(), snapshot.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for SnapshotStatusRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum MgetUrlParams<'a> {
        None,
        Index(Index<'a>),
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> MgetUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                MgetUrlParams::None => Url::from("/_mget"),
                MgetUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(7usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mget");
                    Url::from(url)
                }
                MgetUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(8usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_mget");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct MgetRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> MgetRequest<'a, B> {
        pub fn new(body: B) -> Self {
            MgetRequest {
                url: MgetUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            MgetRequest {
                url: MgetUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            MgetRequest {
                url: MgetUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for MgetRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesExistsTemplateUrlParams<'a> {
        Name(Name<'a>),
    }
    impl<'a> IndicesExistsTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesExistsTemplateUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_template/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesExistsTemplateRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesExistsTemplateRequest<'a> {
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesExistsTemplateRequest {
                url: IndicesExistsTemplateUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesExistsTemplateRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Head,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetUpgradeUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesGetUpgradeUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesGetUpgradeUrlParams::None => Url::from("/_upgrade"),
                IndicesGetUpgradeUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_upgrade");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesGetUpgradeRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesGetUpgradeRequest<'a> {
        pub fn new() -> Self {
            IndicesGetUpgradeRequest {
                url: IndicesGetUpgradeUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetUpgradeRequest {
                url: IndicesGetUpgradeUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesGetUpgradeRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum PutScriptUrlParams<'a> {
        LangId(Lang<'a>, Id<'a>),
    }
    impl<'a> PutScriptUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                PutScriptUrlParams::LangId(ref lang, ref id) => {
                    let mut url = String::with_capacity(11usize + lang.len() + id.len());
                    url.push_str("/_scripts/");
                    url.push_str(lang.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct PutScriptRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> PutScriptRequest<'a, B> {
        pub fn for_lang_id<ILang, IId>(lang: ILang, id: IId, body: B) -> Self
        where
            ILang: Into<Lang<'a>>,
            IId: Into<Id<'a>>,
        {
            PutScriptRequest {
                url: PutScriptUrlParams::LangId(lang.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for PutScriptRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum GetTemplateUrlParams<'a> {
        Id(Id<'a>),
    }
    impl<'a> GetTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                GetTemplateUrlParams::Id(ref id) => {
                    let mut url = String::with_capacity(18usize + id.len());
                    url.push_str("/_search/template/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct GetTemplateRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> GetTemplateRequest<'a> {
        pub fn for_id<IId>(id: IId) -> Self
        where
            IId: Into<Id<'a>>,
        {
            GetTemplateRequest {
                url: GetTemplateUrlParams::Id(id.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for GetTemplateRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesDeleteTemplateUrlParams<'a> {
        Name(Name<'a>),
    }
    impl<'a> IndicesDeleteTemplateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesDeleteTemplateUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(11usize + name.len());
                    url.push_str("/_template/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesDeleteTemplateRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesDeleteTemplateRequest<'a> {
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            IndicesDeleteTemplateRequest {
                url: IndicesDeleteTemplateUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesDeleteTemplateRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndexUrlParams<'a> {
        IndexType(Index<'a>, Type<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> IndexUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndexUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(2usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    Url::from(url)
                }
                IndexUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(3usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndexRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndexRequest<'a, B> {
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndexRequest {
                url: IndexUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            IndexRequest {
                url: IndexUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndexRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesPutSettingsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesPutSettingsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesPutSettingsUrlParams::None => Url::from("/_settings"),
                IndicesPutSettingsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(11usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_settings");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesPutSettingsRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesPutSettingsRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesPutSettingsRequest {
                url: IndicesPutSettingsUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesPutSettingsRequest {
                url: IndicesPutSettingsUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesPutSettingsRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Put,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatTemplatesUrlParams<'a> {
        None,
        Name(Name<'a>),
    }
    impl<'a> CatTemplatesUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatTemplatesUrlParams::None => Url::from("/_cat/templates"),
                CatTemplatesUrlParams::Name(ref name) => {
                    let mut url = String::with_capacity(16usize + name.len());
                    url.push_str("/_cat/templates/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatTemplatesRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatTemplatesRequest<'a> {
        pub fn new() -> Self {
            CatTemplatesRequest {
                url: CatTemplatesUrlParams::None.url(),
            }
        }
        pub fn for_name<IName>(name: IName) -> Self
        where
            IName: Into<Name<'a>>,
        {
            CatTemplatesRequest {
                url: CatTemplatesUrlParams::Name(name.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatTemplatesRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatIndicesUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> CatIndicesUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatIndicesUrlParams::None => Url::from("/_cat/indices"),
                CatIndicesUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(14usize + index.len());
                    url.push_str("/_cat/indices/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatIndicesRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatIndicesRequest<'a> {
        pub fn new() -> Self {
            CatIndicesRequest {
                url: CatIndicesUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            CatIndicesRequest {
                url: CatIndicesUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatIndicesRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterPutSettingsUrlParams {
        None,
    }
    impl ClusterPutSettingsUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                ClusterPutSettingsUrlParams::None => Url::from("/_cluster/settings"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ClusterPutSettingsRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> ClusterPutSettingsRequest<'a, B> {
        pub fn new(body: B) -> Self {
            ClusterPutSettingsRequest {
                url: ClusterPutSettingsUrlParams::None.url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for ClusterPutSettingsRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Put,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum UpdateUrlParams<'a> {
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> UpdateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                UpdateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(11usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_update");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct UpdateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> UpdateRequest<'a, B> {
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            UpdateRequest {
                url: UpdateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for UpdateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesPutAliasUrlParams<'a> {
        IndexName(Index<'a>, Name<'a>),
    }
    impl<'a> IndicesPutAliasUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesPutAliasUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(11usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_aliases/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesPutAliasRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesPutAliasRequest<'a, B> {
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesPutAliasRequest {
                url: IndicesPutAliasUrlParams::IndexName(index.into(), name.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesPutAliasRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatPluginsUrlParams {
        None,
    }
    impl CatPluginsUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                CatPluginsUrlParams::None => Url::from("/_cat/plugins"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatPluginsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatPluginsRequest<'a> {
        pub fn new() -> Self {
            CatPluginsRequest {
                url: CatPluginsUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatPluginsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CountPercolateUrlParams<'a> {
        IndexType(Index<'a>, Type<'a>),
        IndexTypeId(Index<'a>, Type<'a>, Id<'a>),
    }
    impl<'a> CountPercolateUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CountPercolateUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(19usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/_percolate/count");
                    Url::from(url)
                }
                CountPercolateUrlParams::IndexTypeId(ref index, ref ty, ref id) => {
                    let mut url = String::with_capacity(20usize + index.len() + ty.len() + id.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(ty.as_ref());
                    url.push_str("/");
                    url.push_str(id.as_ref());
                    url.push_str("/_percolate/count");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CountPercolateRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> CountPercolateRequest<'a, B> {
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            CountPercolateRequest {
                url: CountPercolateUrlParams::IndexType(index.into(), ty.into()).url(),
                body: body,
            }
        }
        pub fn for_index_ty_id<IIndex, IType, IId>(index: IIndex, ty: IType, id: IId, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
            IId: Into<Id<'a>>,
        {
            CountPercolateRequest {
                url: CountPercolateUrlParams::IndexTypeId(index.into(), ty.into(), id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for CountPercolateRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesUpgradeUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesUpgradeUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesUpgradeUrlParams::None => Url::from("/_upgrade"),
                IndicesUpgradeUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_upgrade");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesUpgradeRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesUpgradeRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesUpgradeRequest {
                url: IndicesUpgradeUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesUpgradeRequest {
                url: IndicesUpgradeUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesUpgradeRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesDeleteAliasUrlParams<'a> {
        IndexName(Index<'a>, Name<'a>),
    }
    impl<'a> IndicesDeleteAliasUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesDeleteAliasUrlParams::IndexName(ref index, ref name) => {
                    let mut url = String::with_capacity(11usize + index.len() + name.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_aliases/");
                    url.push_str(name.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesDeleteAliasRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesDeleteAliasRequest<'a> {
        pub fn for_index_name<IIndex, IName>(index: IIndex, name: IName) -> Self
        where
            IIndex: Into<Index<'a>>,
            IName: Into<Name<'a>>,
        {
            IndicesDeleteAliasRequest {
                url: IndicesDeleteAliasUrlParams::IndexName(index.into(), name.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesDeleteAliasRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Delete,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatTasksUrlParams {
        None,
    }
    impl CatTasksUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                CatTasksUrlParams::None => Url::from("/_cat/tasks"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatTasksRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatTasksRequest<'a> {
        pub fn new() -> Self {
            CatTasksRequest {
                url: CatTasksUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatTasksRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesRolloverUrlParams<'a> {
        Alias(Alias<'a>),
        AliasNewIndex(Alias<'a>, NewIndex<'a>),
    }
    impl<'a> IndicesRolloverUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesRolloverUrlParams::Alias(ref alias) => {
                    let mut url = String::with_capacity(11usize + alias.len());
                    url.push_str("/");
                    url.push_str(alias.as_ref());
                    url.push_str("/_rollover");
                    Url::from(url)
                }
                IndicesRolloverUrlParams::AliasNewIndex(ref alias, ref new_index) => {
                    let mut url = String::with_capacity(12usize + alias.len() + new_index.len());
                    url.push_str("/");
                    url.push_str(alias.as_ref());
                    url.push_str("/_rollover/");
                    url.push_str(new_index.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesRolloverRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesRolloverRequest<'a, B> {
        pub fn for_alias<IAlias>(alias: IAlias, body: B) -> Self
        where
            IAlias: Into<Alias<'a>>,
        {
            IndicesRolloverRequest {
                url: IndicesRolloverUrlParams::Alias(alias.into()).url(),
                body: body,
            }
        }
        pub fn for_alias_new_index<IAlias, INewIndex>(alias: IAlias, new_index: INewIndex, body: B) -> Self
        where
            IAlias: Into<Alias<'a>>,
            INewIndex: Into<NewIndex<'a>>,
        {
            IndicesRolloverRequest {
                url: IndicesRolloverUrlParams::AliasNewIndex(alias.into(), new_index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesRolloverRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ReindexRethrottleUrlParams<'a> {
        TaskId(TaskId<'a>),
    }
    impl<'a> ReindexRethrottleUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                ReindexRethrottleUrlParams::TaskId(ref task_id) => {
                    let mut url = String::with_capacity(30usize + task_id.len());
                    url.push_str("/_delete_by_query/");
                    url.push_str(task_id.as_ref());
                    url.push_str("/_rethrottle");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ReindexRethrottleRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> ReindexRethrottleRequest<'a, B> {
        pub fn for_task_id<ITaskId>(task_id: ITaskId, body: B) -> Self
        where
            ITaskId: Into<TaskId<'a>>,
        {
            ReindexRethrottleRequest {
                url: ReindexRethrottleUrlParams::TaskId(task_id.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for ReindexRethrottleRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum SnapshotCreateRepositoryUrlParams<'a> {
        Repository(Repository<'a>),
    }
    impl<'a> SnapshotCreateRepositoryUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                SnapshotCreateRepositoryUrlParams::Repository(ref repository) => {
                    let mut url = String::with_capacity(11usize + repository.len());
                    url.push_str("/_snapshot/");
                    url.push_str(repository.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SnapshotCreateRepositoryRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> SnapshotCreateRepositoryRequest<'a, B> {
        pub fn for_repository<IRepository>(repository: IRepository, body: B) -> Self
        where
            IRepository: Into<Repository<'a>>,
        {
            SnapshotCreateRepositoryRequest {
                url: SnapshotCreateRepositoryUrlParams::Repository(repository.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for SnapshotCreateRepositoryRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesGetUrlParams<'a> {
        Index(Index<'a>),
        IndexFeature(Index<'a>, Feature<'a>),
    }
    impl<'a> IndicesGetUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesGetUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(1usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    Url::from(url)
                }
                IndicesGetUrlParams::IndexFeature(ref index, ref feature) => {
                    let mut url = String::with_capacity(2usize + index.len() + feature.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/");
                    url.push_str(feature.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesGetRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesGetRequest<'a> {
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesGetRequest {
                url: IndicesGetUrlParams::Index(index.into()).url(),
            }
        }
        pub fn for_index_feature<IIndex, IFeature>(index: IIndex, feature: IFeature) -> Self
        where
            IIndex: Into<Index<'a>>,
            IFeature: Into<Feature<'a>>,
        {
            IndicesGetRequest {
                url: IndicesGetUrlParams::IndexFeature(index.into(), feature.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesGetRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesAnalyzeUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesAnalyzeUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesAnalyzeUrlParams::None => Url::from("/_analyze"),
                IndicesAnalyzeUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(10usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_analyze");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesAnalyzeRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesAnalyzeRequest<'a, B> {
        pub fn new(body: B) -> Self {
            IndicesAnalyzeRequest {
                url: IndicesAnalyzeUrlParams::None.url(),
                body: body,
            }
        }
        pub fn for_index<IIndex>(index: IIndex, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesAnalyzeRequest {
                url: IndicesAnalyzeUrlParams::Index(index.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesAnalyzeRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatFielddataUrlParams<'a> {
        None,
        Fields(Fields<'a>),
    }
    impl<'a> CatFielddataUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                CatFielddataUrlParams::None => Url::from("/_cat/fielddata"),
                CatFielddataUrlParams::Fields(ref fields) => {
                    let mut url = String::with_capacity(16usize + fields.len());
                    url.push_str("/_cat/fielddata/");
                    url.push_str(fields.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatFielddataRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatFielddataRequest<'a> {
        pub fn new() -> Self {
            CatFielddataRequest {
                url: CatFielddataUrlParams::None.url(),
            }
        }
        pub fn for_fields<IFields>(fields: IFields) -> Self
        where
            IFields: Into<Fields<'a>>,
        {
            CatFielddataRequest {
                url: CatFielddataUrlParams::Fields(fields.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatFielddataRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesSegmentsUrlParams<'a> {
        None,
        Index(Index<'a>),
    }
    impl<'a> IndicesSegmentsUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesSegmentsUrlParams::None => Url::from("/_segments"),
                IndicesSegmentsUrlParams::Index(ref index) => {
                    let mut url = String::with_capacity(11usize + index.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_segments");
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesSegmentsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesSegmentsRequest<'a> {
        pub fn new() -> Self {
            IndicesSegmentsRequest {
                url: IndicesSegmentsUrlParams::None.url(),
            }
        }
        pub fn for_index<IIndex>(index: IIndex) -> Self
        where
            IIndex: Into<Index<'a>>,
        {
            IndicesSegmentsRequest {
                url: IndicesSegmentsUrlParams::Index(index.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesSegmentsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesShrinkUrlParams<'a> {
        IndexTarget(Index<'a>, Target<'a>),
    }
    impl<'a> IndicesShrinkUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesShrinkUrlParams::IndexTarget(ref index, ref target) => {
                    let mut url = String::with_capacity(10usize + index.len() + target.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_shrink/");
                    url.push_str(target.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesShrinkRequest<'a, B> {
        pub url: Url<'a>,
        pub body: B,
    }
    impl<'a, B> IndicesShrinkRequest<'a, B> {
        pub fn for_index_target<IIndex, ITarget>(index: IIndex, target: ITarget, body: B) -> Self
        where
            IIndex: Into<Index<'a>>,
            ITarget: Into<Target<'a>>,
        {
            IndicesShrinkRequest {
                url: IndicesShrinkUrlParams::IndexTarget(index.into(), target.into()).url(),
                body: body,
            }
        }
    }
    impl<'a, B> Into<HttpRequest<'a, B>> for IndicesShrinkRequest<'a, B> {
        fn into(self) -> HttpRequest<'a, B> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Post,
                body: Some(self.body),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum TasksListUrlParams {
        None,
    }
    impl TasksListUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                TasksListUrlParams::None => Url::from("/_tasks"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct TasksListRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> TasksListRequest<'a> {
        pub fn new() -> Self {
            TasksListRequest {
                url: TasksListUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for TasksListRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum CatMasterUrlParams {
        None,
    }
    impl CatMasterUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                CatMasterUrlParams::None => Url::from("/_cat/master"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct CatMasterRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> CatMasterRequest<'a> {
        pub fn new() -> Self {
            CatMasterRequest {
                url: CatMasterUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for CatMasterRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum IndicesExistsTypeUrlParams<'a> {
        IndexType(Index<'a>, Type<'a>),
    }
    impl<'a> IndicesExistsTypeUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                IndicesExistsTypeUrlParams::IndexType(ref index, ref ty) => {
                    let mut url = String::with_capacity(11usize + index.len() + ty.len());
                    url.push_str("/");
                    url.push_str(index.as_ref());
                    url.push_str("/_mapping/");
                    url.push_str(ty.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct IndicesExistsTypeRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> IndicesExistsTypeRequest<'a> {
        pub fn for_index_ty<IIndex, IType>(index: IIndex, ty: IType) -> Self
        where
            IIndex: Into<Index<'a>>,
            IType: Into<Type<'a>>,
        {
            IndicesExistsTypeRequest {
                url: IndicesExistsTypeUrlParams::IndexType(index.into(), ty.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for IndicesExistsTypeRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Head,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum ClusterGetSettingsUrlParams {
        None,
    }
    impl ClusterGetSettingsUrlParams {
        pub fn url<'a>(self) -> Url<'a> {
            match self {
                ClusterGetSettingsUrlParams::None => Url::from("/_cluster/settings"),
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct ClusterGetSettingsRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> ClusterGetSettingsRequest<'a> {
        pub fn new() -> Self {
            ClusterGetSettingsRequest {
                url: ClusterGetSettingsUrlParams::None.url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for ClusterGetSettingsRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    enum NodesInfoUrlParams<'a> {
        None,
        Metric(Metric<'a>),
        NodeId(NodeId<'a>),
        NodeIdMetric(NodeId<'a>, Metric<'a>),
    }
    impl<'a> NodesInfoUrlParams<'a> {
        pub fn url(self) -> Url<'a> {
            match self {
                NodesInfoUrlParams::None => Url::from("/_nodes"),
                NodesInfoUrlParams::Metric(ref metric) => {
                    let mut url = String::with_capacity(8usize + metric.len());
                    url.push_str("/_nodes/");
                    url.push_str(metric.as_ref());
                    Url::from(url)
                }
                NodesInfoUrlParams::NodeId(ref node_id) => {
                    let mut url = String::with_capacity(8usize + node_id.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    Url::from(url)
                }
                NodesInfoUrlParams::NodeIdMetric(ref node_id, ref metric) => {
                    let mut url = String::with_capacity(9usize + node_id.len() + metric.len());
                    url.push_str("/_nodes/");
                    url.push_str(node_id.as_ref());
                    url.push_str("/");
                    url.push_str(metric.as_ref());
                    Url::from(url)
                }
            }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct NodesInfoRequest<'a> {
        pub url: Url<'a>,
    }
    impl<'a> NodesInfoRequest<'a> {
        pub fn new() -> Self {
            NodesInfoRequest {
                url: NodesInfoUrlParams::None.url(),
            }
        }
        pub fn for_metric<IMetric>(metric: IMetric) -> Self
        where
            IMetric: Into<Metric<'a>>,
        {
            NodesInfoRequest {
                url: NodesInfoUrlParams::Metric(metric.into()).url(),
            }
        }
        pub fn for_node_id<INodeId>(node_id: INodeId) -> Self
        where
            INodeId: Into<NodeId<'a>>,
        {
            NodesInfoRequest {
                url: NodesInfoUrlParams::NodeId(node_id.into()).url(),
            }
        }
        pub fn for_node_id_metric<INodeId, IMetric>(node_id: INodeId, metric: IMetric) -> Self
        where
            INodeId: Into<NodeId<'a>>,
            IMetric: Into<Metric<'a>>,
        {
            NodesInfoRequest {
                url: NodesInfoUrlParams::NodeIdMetric(node_id.into(), metric.into()).url(),
            }
        }
    }
    impl<'a> Into<HttpRequest<'a, DefaultBody>> for NodesInfoRequest<'a> {
        fn into(self) -> HttpRequest<'a, DefaultBody> {
            HttpRequest {
                url: self.url,
                method: HttpMethod::Get,
                body: None,
            }
        }
    }
}

pub mod http {
    use std::borrow::Cow;
    use std::ops::Deref;

    #[derive(Debug, PartialEq, Clone)]
    #[doc = r" A wrapper around an owned or borrowed url."]
    pub struct Url<'a>(Cow<'a, str>);
    impl<'a> From<&'a str> for Url<'a> {
        fn from(value: &'a str) -> Url<'a> {
            Url(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Url<'a> {
        fn from(value: String) -> Url<'a> {
            Url(Cow::Owned(value))
        }
    }
    impl<'a> Deref for Url<'a> {
        type Target = Cow<'a, str>;
        fn deref(&self) -> &Cow<'a, str> {
            &self.0
        }
    }
    #[doc = r" A default body type."]
    pub type DefaultBody = &'static [u8];
    #[doc = r" A convenience method for a default, empty body."]
    #[doc = r" This method doesn't allocate."]
    pub fn empty_body() -> DefaultBody {
        &[]
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = r" A general request type that all endpoints can be converted into."]
    pub struct HttpRequest<'a, B> {
        pub url: Url<'a>,
        pub method: HttpMethod,
        pub body: Option<B>,
    }
    #[derive(Debug, PartialEq, Clone)]
    #[doc = r" A standard HTTP verb."]
    pub enum HttpMethod {
        Head,
        Get,
        Post,
        Put,
        Delete,
        Patch,
    }
}

pub mod params {
    use std::borrow::Cow;

    include!("genned.params.rs");

    #[derive(Debug, PartialEq, Clone)]
    pub struct Alias<'a>(pub Cow<'a, str>);
    pub fn alias<'a, I>(value: I) -> Alias<'a>
    where
        I: Into<Alias<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Alias<'a> {
        fn from(value: &'a str) -> Alias<'a> {
            Alias(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Alias<'a> {
        fn from(value: String) -> Alias<'a> {
            Alias(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Alias<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Feature<'a>(pub Cow<'a, str>);
    pub fn feature<'a, I>(value: I) -> Feature<'a>
    where
        I: Into<Feature<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Feature<'a> {
        fn from(value: &'a str) -> Feature<'a> {
            Feature(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Feature<'a> {
        fn from(value: String) -> Feature<'a> {
            Feature(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Feature<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Fields<'a>(pub Cow<'a, str>);
    pub fn fields<'a, I>(value: I) -> Fields<'a>
    where
        I: Into<Fields<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Fields<'a> {
        fn from(value: &'a str) -> Fields<'a> {
            Fields(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Fields<'a> {
        fn from(value: String) -> Fields<'a> {
            Fields(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Fields<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Id<'a>(pub Cow<'a, str>);
    pub fn id<'a, I>(value: I) -> Id<'a>
    where
        I: Into<Id<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Id<'a> {
        fn from(value: &'a str) -> Id<'a> {
            Id(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Id<'a> {
        fn from(value: String) -> Id<'a> {
            Id(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Id<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Index<'a>(pub Cow<'a, str>);
    pub fn index<'a, I>(value: I) -> Index<'a>
    where
        I: Into<Index<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Index<'a> {
        fn from(value: &'a str) -> Index<'a> {
            Index(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Index<'a> {
        fn from(value: String) -> Index<'a> {
            Index(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Index<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct IndexMetric<'a>(pub Cow<'a, str>);
    pub fn index_metric<'a, I>(value: I) -> IndexMetric<'a>
    where
        I: Into<IndexMetric<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for IndexMetric<'a> {
        fn from(value: &'a str) -> IndexMetric<'a> {
            IndexMetric(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for IndexMetric<'a> {
        fn from(value: String) -> IndexMetric<'a> {
            IndexMetric(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for IndexMetric<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Lang<'a>(pub Cow<'a, str>);
    pub fn lang<'a, I>(value: I) -> Lang<'a>
    where
        I: Into<Lang<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Lang<'a> {
        fn from(value: &'a str) -> Lang<'a> {
            Lang(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Lang<'a> {
        fn from(value: String) -> Lang<'a> {
            Lang(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Lang<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Metric<'a>(pub Cow<'a, str>);
    pub fn metric<'a, I>(value: I) -> Metric<'a>
    where
        I: Into<Metric<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Metric<'a> {
        fn from(value: &'a str) -> Metric<'a> {
            Metric(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Metric<'a> {
        fn from(value: String) -> Metric<'a> {
            Metric(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Metric<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Name<'a>(pub Cow<'a, str>);
    pub fn name<'a, I>(value: I) -> Name<'a>
    where
        I: Into<Name<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Name<'a> {
        fn from(value: &'a str) -> Name<'a> {
            Name(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Name<'a> {
        fn from(value: String) -> Name<'a> {
            Name(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Name<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct NewIndex<'a>(pub Cow<'a, str>);
    pub fn new_index<'a, I>(value: I) -> NewIndex<'a>
    where
        I: Into<NewIndex<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for NewIndex<'a> {
        fn from(value: &'a str) -> NewIndex<'a> {
            NewIndex(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for NewIndex<'a> {
        fn from(value: String) -> NewIndex<'a> {
            NewIndex(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for NewIndex<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct NodeId<'a>(pub Cow<'a, str>);
    pub fn node_id<'a, I>(value: I) -> NodeId<'a>
    where
        I: Into<NodeId<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for NodeId<'a> {
        fn from(value: &'a str) -> NodeId<'a> {
            NodeId(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for NodeId<'a> {
        fn from(value: String) -> NodeId<'a> {
            NodeId(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for NodeId<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Repository<'a>(pub Cow<'a, str>);
    pub fn repository<'a, I>(value: I) -> Repository<'a>
    where
        I: Into<Repository<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Repository<'a> {
        fn from(value: &'a str) -> Repository<'a> {
            Repository(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Repository<'a> {
        fn from(value: String) -> Repository<'a> {
            Repository(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Repository<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ScrollId<'a>(pub Cow<'a, str>);
    pub fn scroll_id<'a, I>(value: I) -> ScrollId<'a>
    where
        I: Into<ScrollId<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for ScrollId<'a> {
        fn from(value: &'a str) -> ScrollId<'a> {
            ScrollId(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for ScrollId<'a> {
        fn from(value: String) -> ScrollId<'a> {
            ScrollId(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for ScrollId<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Snapshot<'a>(pub Cow<'a, str>);
    pub fn snapshot<'a, I>(value: I) -> Snapshot<'a>
    where
        I: Into<Snapshot<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Snapshot<'a> {
        fn from(value: &'a str) -> Snapshot<'a> {
            Snapshot(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Snapshot<'a> {
        fn from(value: String) -> Snapshot<'a> {
            Snapshot(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Snapshot<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Target<'a>(pub Cow<'a, str>);
    pub fn target<'a, I>(value: I) -> Target<'a>
    where
        I: Into<Target<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Target<'a> {
        fn from(value: &'a str) -> Target<'a> {
            Target(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Target<'a> {
        fn from(value: String) -> Target<'a> {
            Target(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Target<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct TaskId<'a>(pub Cow<'a, str>);
    pub fn task_id<'a, I>(value: I) -> TaskId<'a>
    where
        I: Into<TaskId<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for TaskId<'a> {
        fn from(value: &'a str) -> TaskId<'a> {
            TaskId(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for TaskId<'a> {
        fn from(value: String) -> TaskId<'a> {
            TaskId(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for TaskId<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ThreadPoolPatterns<'a>(pub Cow<'a, str>);
    pub fn thread_pool_patterns<'a, I>(value: I) -> ThreadPoolPatterns<'a>
    where
        I: Into<ThreadPoolPatterns<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for ThreadPoolPatterns<'a> {
        fn from(value: &'a str) -> ThreadPoolPatterns<'a> {
            ThreadPoolPatterns(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for ThreadPoolPatterns<'a> {
        fn from(value: String) -> ThreadPoolPatterns<'a> {
            ThreadPoolPatterns(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for ThreadPoolPatterns<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Type<'a>(pub Cow<'a, str>);
    pub fn ty<'a, I>(value: I) -> Type<'a>
    where
        I: Into<Type<'a>>,
    {
        value.into()
    }
    impl<'a> From<&'a str> for Type<'a> {
        fn from(value: &'a str) -> Type<'a> {
            Type(Cow::Borrowed(value))
        }
    }
    impl<'a> From<String> for Type<'a> {
        fn from(value: String) -> Type<'a> {
            Type(Cow::Owned(value))
        }
    }
    impl<'a> ::std::ops::Deref for Type<'a> {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

}