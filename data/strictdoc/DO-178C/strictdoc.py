from strictdoc.backend.sdoc.models.document_config import DocumentConfig
from strictdoc.backend.sdoc.models.document_grammar import DocumentGrammar
from strictdoc.export.html.html_generator import HTMLGenerator

project_config = DocumentConfig(
    project_title="DO-178C Flight Management System Certification",
    project_version="2.1.0",
)

# Document tree structure
documents = [
    {
        "path": "plans/PSAC/PSAC.sdoc",
        "title": "Plan for Software Aspects of Certification"
    },
    {
        "path": "plans/SVP/SVP.sdoc",
        "title": "Software Verification Plan"
    },
    {
        "path": "requirements/SRD/SRD.sdoc",
        "title": "Software Requirements Data"
    },
    {
        "path": "requirements/SDD/SDD.sdoc",
        "title": "Software Design Description"
    },
    {
        "path": "verification/SVCP/SVCP.sdoc",
        "title": "Software Verification Cases and Procedures"
    },
    {
        "path": "SAS/SAS.sdoc",
        "title": "Software Accomplishment Summary"
    },
]
