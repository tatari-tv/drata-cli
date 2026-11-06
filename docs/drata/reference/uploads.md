Title: Uploads

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Uploads/

Markdown Content:
[](https://developers.drata.com/openapi/reference/v2/tag/Uploads/#tag/Uploads/operation/UploadsPublicV2Controller_requestUploadUrl!path=purpose&t=request)purpose
required string

What the uploaded file will be attached to. Determines authorization, storage prefix, and workspace requirements.

Value:"evidence"
[](https://developers.drata.com/openapi/reference/v2/tag/Uploads/#tag/Uploads/operation/UploadsPublicV2Controller_requestUploadUrl!path=workspaceId&t=request)workspaceId number

Required when `purpose` is workspace-scoped (currently: `evidence`). Omit for account-global purposes.
[](https://developers.drata.com/openapi/reference/v2/tag/Uploads/#tag/Uploads/operation/UploadsPublicV2Controller_requestUploadUrl!path=fileName&t=request)fileName string<= 191 characters

The original filename of the file that will be uploaded. When provided, it is used to build the human-readable segment of the S3 object key. Optional for browser-driven uploads (`includeWebUploadUrl: true`) where the user picks the file after the URL is minted — in that case the actual filename lands on the evidence artifact once the browser PUT completes.
[](https://developers.drata.com/openapi/reference/v2/tag/Uploads/#tag/Uploads/operation/UploadsPublicV2Controller_requestUploadUrl!path=contentType&t=request)contentType string

The MIME type of the file being uploaded. When provided, sent back as the `Content-Type` on the pre-signed URL, so the client's PUT must send a matching `Content-Type` header. When omitted, the URL is signed WITHOUT binding a Content-Type — the client can PUT any mime (useful for browser flows where the mime is not known at mint time).
[](https://developers.drata.com/openapi/reference/v2/tag/Uploads/#tag/Uploads/operation/UploadsPublicV2Controller_requestUploadUrl!path=includeWebUploadUrl&t=request)includeWebUploadUrl boolean

When `true`, the response additionally includes a `webUploadUrl` — a short-lived, browser-facing URL that hosts a drag-and-drop upload page backed by this pre-signed URL. Intended for surfaces (e.g., MCP tools in Claude Desktop) that want to hand the user a clickable link rather than a curl command. Defaults to `false`.
[](https://developers.drata.com/openapi/reference/v2/tag/Uploads/#tag/Uploads/operation/UploadsPublicV2Controller_requestUploadUrl!path=existingArtifactId&t=request)existingArtifactId number

When set, the browser upload session is bound as a REPLACE of the referenced artifact. When the user completes the upload, the API bumps the existing artifact chain (same artifact ID, new version, previous version archived) instead of creating a new artifact. If the user abandons the upload, no changes to the evidence occur -- the previous version stays current. Validated at mint time to ensure the artifact exists, belongs to the caller, and is a current S3_FILE. Only meaningful with `includeWebUploadUrl: true` (browser flow).
[](https://developers.drata.com/openapi/reference/v2/tag/Uploads/#tag/Uploads/operation/UploadsPublicV2Controller_requestUploadUrl!path=addToEvidenceId&t=request)addToEvidenceId number

When set, the browser upload session is bound as an ADD of a new artifact to the referenced evidence. When the user completes the upload, the API attaches a new S3_FILE artifact to the evidence -- no artifact row is created at mint time. If the user abandons the upload, no artifact appears on the evidence (no phantom placeholder). Validated at mint time to ensure the evidence exists and belongs to the workspace. Mutually exclusive with `existingArtifactId`. Only meaningful with `includeWebUploadUrl: true` (browser flow).
[](https://developers.drata.com/openapi/reference/v2/tag/Uploads/#tag/Uploads/operation/UploadsPublicV2Controller_requestUploadUrl!path=multiArtifact&t=request)multiArtifact boolean

When `true`, the browser upload session renders the multi-artifact cart page (drag N files + add N URLs, then Confirm) instead of the single-file page. Requires `includeWebUploadUrl: true`. Incompatible with `existingArtifactId` (REPLACE is single-file only). Must be paired with either `addToEvidenceId` (add-many to existing evidence) or `createEvidence` (create new evidence with N artifacts).
object

Evidence-level metadata for CREATE-with-N. Required when `multiArtifact: true` and `addToEvidenceId` is omitted; forbidden otherwise. Stashed on the session at mint time and consumed at commit time to drive `POST /evidence-library`.
