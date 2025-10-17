using Imager.Common;
using Imager.Responses;
using Microsoft.Extensions.Options;

namespace Imager.Endpoints;

internal static class GetImagesEndpoint
{
    public static void MapGetImagesEndpoint(this WebApplication app)
    {
        app.MapGet("/images", (IOptions<ImagerOptions> imagerOptions) =>
        {
            var response = Directory.GetFiles(imagerOptions.Value.Directory).Select(path => new ImageResponse
            {
                FilePath = path,
                Url = $"/images/{Path.GetFileNameWithoutExtension(path)}"
            });
            
            return Results.Ok(response);
        });
    }
}