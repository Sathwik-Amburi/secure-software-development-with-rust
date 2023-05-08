import os
import shutil
import time

def is_file_open(filename):
    """Check if a file is open by trying to open it in write mode"""
    try:
        with open(filename, 'w') as f:
            pass
    except IOError:
        return True
    else:
        return False

def main():
    target_file = 'example.txt'
    sensitive_file = 'sensitive.txt'
    temp_file = 'temp.txt'

    while True:
        # Check if the target file exists and is not open
        if os.path.exists(target_file) and not is_file_open(target_file):
            # Copy the contents of the sensitive file to a temporary file
            shutil.copyfile(sensitive_file, temp_file)

            # Replace the target file with the temporary file
            os.rename(temp_file, target_file)

            # Exit the loop after the change has been made
            break

        time.sleep(0.1)

if __name__ == '__main__':
    main()
