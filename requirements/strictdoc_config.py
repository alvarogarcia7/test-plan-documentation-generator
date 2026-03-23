from strictdoc.backend.sdoc.models.document_config import DocumentConfig
from strictdoc.backend.sdoc.models.project_config import ProjectConfig

project_config = ProjectConfig(
    project_title="Test Plan Documentation Generator",
    dir_for_sdoc_files="requirements",
    output_dir="requirements/output",
    enable_traceability=True,
    include_doc_types=["REQUIREMENT", "SECTION"],
    source_root_path_for_code_traceability="src",
)
