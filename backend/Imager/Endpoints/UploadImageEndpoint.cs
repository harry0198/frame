using Imager.Common;
using Imager.Responses;
using Microsoft.Extensions.Options;

namespace Imager.Endpoints;

internal static class UploadImageEndpoint
{
    public static void MapUploadImageEndpoint(this WebApplication app)
    {
        app.MapPost("/upload", async (IFormFile file, IOptions<ImagerOptions> options) =>
        {
            if (file.Length == 0)
            {
                return Results.BadRequest("No file uploaded.");
            }

            var originalName = Path.GetFileName(file.FileName);
            var ext = Path.GetExtension(originalName);
            var newFileName = $"{Guid.NewGuid():N}{ext}";
            var savedPath = Path.Combine(options.Value.Directory, newFileName);

            await using var stream = File.Create(savedPath);
            await file.CopyToAsync(stream);

            return Results.Created($"/images/{newFileName}", new ImageUploadResponse(FileName: newFileName, Size: file.Length));
        }).DisableAntiforgery();
    }
}