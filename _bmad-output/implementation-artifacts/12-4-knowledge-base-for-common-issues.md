# Story 12.4: Knowledge Base for Common Issues

Status: ready-for-dev

## Story

As a support person (Sofia),
I want a knowledge base of solutions,
So that I can resolve issues quickly.

## Acceptance Criteria

1. **Given** support person is investigating
   **When** they access knowledge base
   **Then** searchable list of common issues is shown

2. **Given** knowledge base entry exists
   **When** viewed
   **Then** each entry has: problem, cause, solution

3. **Given** knowledge base entry exists
   **When** viewed
   **Then** related error messages are shown

4. **Given** knowledge base entry exists
   **When** viewed
   **Then** steps to resolve are included

5. **Given** knowledge base exists
   **When** support has new solution
   **Then** can add new entries

6. **Given** error is being viewed
   **When** knowledge base searched
   **Then** can link issues to knowledge base entries

7. **Given** knowledge base is used
   **When** sorted
   **Then** most viewed entries are shown first

## Tasks

- [ ] Task 1: Create knowledge_base table
- [ ] Task 2: Create KnowledgeBasePage component
- [ ] Task 3: Create KBArticle component
- [ ] Task 4: Create KBSearchBar component
- [ ] Task 5: Create AddArticleDialog
- [ ] Task 6: Implement view tracking
- [ ] Task 7: Create error-to-KB linking

## Dev Notes

### Database Schema

```sql
-- migrations/20260103_create_knowledge_base.sql
CREATE TABLE knowledge_base_articles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    problem TEXT NOT NULL,
    cause TEXT,
    solution TEXT NOT NULL,
    steps JSONB, -- Array of step objects
    related_errors TEXT[], -- Error messages that match
    tags TEXT[],
    view_count INTEGER DEFAULT 0,
    helpful_count INTEGER DEFAULT 0,
    not_helpful_count INTEGER DEFAULT 0,
    created_by VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_kb_articles_tags ON knowledge_base_articles USING GIN(tags);
CREATE INDEX idx_kb_articles_errors ON knowledge_base_articles USING GIN(related_errors);
CREATE INDEX idx_kb_articles_view_count ON knowledge_base_articles(view_count DESC);

-- Full-text search
CREATE INDEX idx_kb_articles_search ON knowledge_base_articles 
    USING GIN(to_tsvector('english', title || ' ' || problem || ' ' || solution));
```

### Article Types

```rust
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeBaseArticle {
    pub id: Uuid,
    pub title: String,
    pub problem: String,
    pub cause: Option<String>,
    pub solution: String,
    pub steps: Option<Vec<SolutionStep>>,
    pub related_errors: Vec<String>,
    pub tags: Vec<String>,
    pub view_count: i32,
    pub helpful_count: i32,
    pub not_helpful_count: i32,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolutionStep {
    pub order: i32,
    pub description: String,
    pub code: Option<String>,
    pub screenshot_url: Option<String>,
}
```

### Knowledge Base API

```rust
// GET /api/v1/knowledge-base
pub async fn search_articles(
    State(state): State<Arc<AppState>>,
    Query(query): Query<KBSearchQuery>,
) -> Result<Json<Vec<KnowledgeBaseArticle>>, ApiError> {
    let results = if let Some(search) = &query.search {
        sqlx::query_as::<_, KnowledgeBaseArticle>(
            r#"
            SELECT * FROM knowledge_base_articles
            WHERE to_tsvector('english', title || ' ' || problem || ' ' || solution) 
                  @@ plainto_tsquery('english', $1)
            ORDER BY 
                ts_rank(to_tsvector('english', title || ' ' || problem || ' ' || solution), 
                        plainto_tsquery('english', $1)) DESC,
                view_count DESC
            LIMIT 50
            "#,
        )
        .bind(search)
        .fetch_all(&state.db_pool)
        .await?
    } else {
        sqlx::query_as::<_, KnowledgeBaseArticle>(
            "SELECT * FROM knowledge_base_articles ORDER BY view_count DESC LIMIT 50"
        )
        .fetch_all(&state.db_pool)
        .await?
    };

    Ok(Json(results))
}

// GET /api/v1/knowledge-base/match
pub async fn match_error(
    State(state): State<Arc<AppState>>,
    Query(query): Query<MatchErrorQuery>,
) -> Result<Json<Vec<KnowledgeBaseArticle>>, ApiError> {
    // Find articles with matching error patterns
    let results = sqlx::query_as::<_, KnowledgeBaseArticle>(
        r#"
        SELECT * FROM knowledge_base_articles
        WHERE EXISTS (
            SELECT 1 FROM unnest(related_errors) AS err
            WHERE $1 ILIKE '%' || err || '%'
        )
        ORDER BY view_count DESC
        LIMIT 5
        "#,
    )
    .bind(&query.error_message)
    .fetch_all(&state.db_pool)
    .await?;

    Ok(Json(results))
}

// POST /api/v1/knowledge-base
pub async fn create_article(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateArticleRequest>,
) -> Result<(StatusCode, Json<KnowledgeBaseArticle>), ApiError> {
    let article = sqlx::query_as::<_, KnowledgeBaseArticle>(
        r#"
        INSERT INTO knowledge_base_articles 
            (title, problem, cause, solution, steps, related_errors, tags, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(&request.title)
    .bind(&request.problem)
    .bind(&request.cause)
    .bind(&request.solution)
    .bind(sqlx::types::Json(&request.steps))
    .bind(&request.related_errors)
    .bind(&request.tags)
    .bind("current_user")
    .fetch_one(&state.db_pool)
    .await?;

    Ok((StatusCode::CREATED, Json(article)))
}

// PUT /api/v1/knowledge-base/:id/view
pub async fn record_view(
    State(state): State<Arc<AppState>>,
    Path(article_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    sqlx::query(
        "UPDATE knowledge_base_articles SET view_count = view_count + 1 WHERE id = $1"
    )
    .bind(article_id)
    .execute(&state.db_pool)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
```

### Knowledge Base Page

```tsx
// frontend/src/pages/admin/KnowledgeBase.tsx
export function KnowledgeBasePage() {
  const [search, setSearch] = useState("");
  const [selectedArticle, setSelectedArticle] = useState<string | null>(null);
  const [showAddDialog, setShowAddDialog] = useState(false);
  
  const { data: articles, isLoading } = useKnowledgeBase(search);

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Knowledge Base</h1>
          <p className="text-sm text-neutral-500">Solutions for common issues</p>
        </div>
        <button
          onClick={() => setShowAddDialog(true)}
          className="px-4 py-2 bg-primary-500 text-white rounded-lg"
        >
          + Add Article
        </button>
      </div>

      {/* Search */}
      <div className="relative max-w-xl">
        <MagnifyingGlassIcon className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-neutral-400" />
        <input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search knowledge base..."
          className="w-full pl-10 pr-4 py-3 border border-neutral-300 rounded-lg"
        />
      </div>

      {/* Articles Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {isLoading ? (
          [...Array(6)].map((_, i) => <ArticleSkeleton key={i} />)
        ) : (
          articles?.map((article) => (
            <ArticleCard
              key={article.id}
              article={article}
              onClick={() => setSelectedArticle(article.id)}
            />
          ))
        )}
      </div>

      {/* Article Detail Dialog */}
      {selectedArticle && (
        <ArticleDetailDialog
          articleId={selectedArticle}
          open={!!selectedArticle}
          onOpenChange={() => setSelectedArticle(null)}
        />
      )}

      {/* Add Article Dialog */}
      <AddArticleDialog
        open={showAddDialog}
        onOpenChange={setShowAddDialog}
      />
    </div>
  );
}
```

### Article Card

```tsx
// frontend/src/components/support/ArticleCard.tsx
interface ArticleCardProps {
  article: KnowledgeBaseArticle;
  onClick: () => void;
}

export function ArticleCard({ article, onClick }: ArticleCardProps) {
  return (
    <button
      onClick={onClick}
      className="text-left p-4 bg-white border border-neutral-200 rounded-xl hover:shadow-md transition-all"
    >
      <h3 className="font-medium text-neutral-900 mb-2">{article.title}</h3>
      <p className="text-sm text-neutral-600 line-clamp-2 mb-3">
        {article.problem}
      </p>
      
      <div className="flex items-center justify-between text-xs text-neutral-400">
        <div className="flex items-center gap-1">
          <EyeOpenIcon className="w-3 h-3" />
          {article.viewCount}
        </div>
        <div className="flex flex-wrap gap-1">
          {article.tags.slice(0, 2).map((tag) => (
            <span key={tag} className="px-2 py-0.5 bg-neutral-100 rounded">
              {tag}
            </span>
          ))}
        </div>
      </div>
    </button>
  );
}
```

### References

- [Source: epics.md#Story 12.4]
