using System.ComponentModel.DataAnnotations;

namespace Imager.Api.Common;

public class ImagerOptions
{
    [Required]
    public required string Directory { get; init; }
}