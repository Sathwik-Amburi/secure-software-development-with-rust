import os
import shutil
import time

def main():
    target_file = 'example.txt'
    sensitive_file = 'sensitive.txt'
    temp_file = 'temp.txt'

    while True:
        # Check if the target file exists
        if os.path.exists(target_file):
            # Copy the contents of the sensitive file to a temporary file
            shutil.copyfile(sensitive_file, temp_file)

            # Replace the target file with the temporary file
            os.rename(temp_file, target_file)

            # Exit the loop after the change has been made
            break

        time.sleep(0.1)

if __name__ == '__main__':
    main()
