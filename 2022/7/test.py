test = {
    "/": []
}

with open("input.txt") as file:
    currentpath = "/"
    for line in file:
        print(currentpath)
        if "ls" in line: continue
        if line[0] == '$':
            if "cd" in line:
                if ".." in line:
                    currentpath = "".join(currentpath.split("/")[:-1]) 
                else:
                    currentpath += line.split(' ')[2]
            elif "dir" in line:
                new_dir = f"{currentpath}{line.split(' ')[1]}"
                test[new_dir] = []
                test[currentpath].append(currentpath)
            else:
                test[currentpath].append(int(line.split(' ')[0]))
print(test)
