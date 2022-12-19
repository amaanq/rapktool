// public static File extractToTmp(String resourcePath, String tmpPrefix, Class<?> clazz) throws BrutException {
//         try {
//             InputStream in = clazz.getResourceAsStream(resourcePath);
//             if (in == null) {
//                 throw new FileNotFoundException(resourcePath);
//             }
//             long suffix = ThreadLocalRandom.current().nextLong();
//             suffix = suffix == Long.MIN_VALUE ? 0 : Math.abs(suffix);
//             File fileOut = File.createTempFile(tmpPrefix, suffix + ".tmp");
//             fileOut.deleteOnExit();
//
//             OutputStream out = new FileOutputStream(fileOut);
//             IOUtils.copy(in, out);
//             in.close();
//             out.close();
//
//             return fileOut;
//         } catch (IOException ex) {
//             throw new BrutException("Could not extract resource: " + resourcePath, ex);
//         }
//     }

use std::fs::File;

use rand::Rng;

pub fn extract_to_tmp(resource_path: &str, tmp_prefix: &str, clazz: &Class) -> Result<File, BrutException> {
    let in_file = clazz.get_resource_as_stream(resource_path)?;
    let suffix = rand::thread_rng().gen::<u64>();
    let out_file = File::create(tmp_prefix.to_owned() + &format!("{}.tmp", suffix))?;
    out_file.delete_on_exit();
    let out = File::create(out_file)?;
    std::io::copy(in_file, out)?;
    Ok(out_file)
}
