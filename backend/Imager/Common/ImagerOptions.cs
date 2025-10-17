using System.ComponentModel.DataAnnotations;

namespace Imager.Common;

public class ImagerOptions
{
    [Required]
    public string Directory { get; set; }
}