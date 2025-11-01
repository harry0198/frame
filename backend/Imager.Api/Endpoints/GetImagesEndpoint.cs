using Imager.Api.Common;
using Imager.Api.Responses;
using Microsoft.Extensions.Options;

namespace Imager.Api.Endpoints;

internal static class GetImagesEndpoint
{
    public static void MapGetImagesEndpoint(this WebApplication app)
    {
        app.MapGet("/images", (IOptions<ImagerOptions> imagerOptions) =>
        {
            var response = Directory.GetFiles(imagerOptions.Value.Directory).Select(path => new ImageResponse(
            
                FilePath: path,
                Url: $"/static/images/{Path.GetFileName(path)}",
                FileNameWithExtension: Path.GetFileName(path)
            ));
            
            return Results.Ok(response);
        });
    }
}