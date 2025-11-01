using Imager.Api.Common;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Options;

namespace Imager.Api.Endpoints;

internal static class DeleteImageEndpoint
{
    public static void MapDeleteImageEndpoint(this WebApplication app)
    {
        app.MapDelete("/images/{file}", (IOptions<ImagerOptions> imagerOptions, [FromRoute] string file) =>
        {
            File.Delete(Path.Combine(imagerOptions.Value.Directory, file));
            
            return Results.NoContent();
        });
    }
}