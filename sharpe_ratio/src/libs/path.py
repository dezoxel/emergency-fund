from os import path

def get_abs_path(*args):
    curr_dir = path.dirname(path.abspath(__file__))
    return path.normpath(path.join(curr_dir, *args))