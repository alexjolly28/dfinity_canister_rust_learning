from pathlib import Path
from upload_scripts.ic_py_canister import get_canister
from upload_scripts.utils import crc32_hash, read_file_bytes, generate_chunks
import mimetypes

# pylint: disable=invalid-name, too-few-public-methods, no-member, too-many-statements


ROOT_PATH = Path(__file__).parent.parent

#  0 - none
#  1 - minimal
#  2 - a lot
DEBUG_VERBOSE = 2


def main():

    canister_name = "storage"
    candid_path = "src/storage/storage.did"
    network = "local"
    canister_storage = get_canister(canister_name, candid_path, network)

    file_path = Path("tests/files/Rick Astley - Never Gonna Give You Up.mp3")

    data = read_file_bytes(file_path)

    chunk_size_mb = 1.9
    chunk_size = int(chunk_size_mb * 1024 * 1024)

    checksum = crc32_hash(data)

    # Iterate over all chunks
    count_bytes = 0
    chunk_ids = []
    for i, chunk in enumerate(generate_chunks(data, chunk_size)):
        count_bytes += len(chunk)
        if DEBUG_VERBOSE == 0:
            pass
        elif DEBUG_VERBOSE == 1:
            print(
                f"chunk size = {len(chunk)} bytes "
                f"({count_bytes / len(data) * 100:.1f}%)"
            )
        else:
            print("+++++++++++++++++++++++++++++++++++++++++++++++++++++")
            print(f"Sending candid for {len(chunk)} bytes :")
            print(f"- i         = {i}")
            print(f"- progress  = {count_bytes / len(data) * 100:.1f}% ")
            print(f"- chunk[0]  = {chunk[0]}")
            print(f"- chunk[-1] = {chunk[-1]}")

        response = canister_storage.upload_chunk(
            {"content": chunk, "order": i}
        )  # pylint: disable=no-member
        print(response)

        chunk_ids.extend(response)

    print(f"checksum: {checksum}")
    print(f"chunk_ids: {chunk_ids}")

    response = canister_storage.chunk_ids_check(chunk_ids)

    print(response)

    asset_filename = file_path.name
    asset_content_type = mimetypes.guess_type(file_path)

    chunk_ids.sort()

    asset_id = canister_storage.commit_batch(
        {
            "file_type": asset_content_type[0],
            "file_name": asset_filename,
            "chunk_ids": chunk_ids,
            "checksum": checksum,
            "content_encoding": {"Identity": None},
        }
    )

    print(f"asset_id: {asset_id}")

    checksum = 0

    asset = canister_storage.query_asset(asset_id[0])

    print(f"asset: {asset}")


if __name__ == "__main__":
    main()
