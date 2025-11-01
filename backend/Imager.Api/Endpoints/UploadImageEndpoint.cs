using Imager.Api.Common;
using Imager.Api.Responses;
using Imager.Api.Validation;
using Microsoft.Extensions.Options;
using SixLabors.ImageSharp;

namespace Imager.Api.Endpoints;

internal static class UploadImageEndpoint
{
    public static void MapUploadImageEndpoint(this WebApplication app)
    {
        app.MapPost("/upload", async (IFormFile file, IOptions<ImagerOptions> options, CancellationToken cancellation) =>
        {
            if (file.Length == 0)
            {
                return Results.BadRequest("No file uploaded.");
            }

            var originalName = Path.GetFileName(file.FileName);
            var ext = Path.GetExtension(originalName);
            var supportedFileType = ext.ToSupportedFileType();
            
            if (!supportedFileType.HasValue || await ImageValidation.IsValidImageSignature(file, supportedFileType.Value, cancellation))
            {
                return Results.BadRequest("Unsupported file type.");
            }
            
            var newFileName = $"{Guid.NewGuid():N}{ext}";
            var savedPath = Path.Combine(options.Value.Directory, newFileName);
            
            // Load image and reencode the image to remove any hostile metadata
            using var image = await Image.LoadAsync(file.OpenReadStream(), cancellation);
            await image.SaveAsync(savedPath, cancellation);

            return Results.Created($"/images/{newFileName}", new ImageUploadResponse(FileName: newFileName, Size: file.Length));
        }).DisableAntiforgery();
    }
}