using System.ComponentModel.DataAnnotations;

namespace Imager.Api.Common;

public class ImagerOptions
{
    [Required]
    public required string Directory { get; init; }
    
    [Required]
    public required string InkyExecutable { get; init; }
    
    [Required]
    public required TimeSpan InkyTimeout { get; init; }
}