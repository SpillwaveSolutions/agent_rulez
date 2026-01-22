# PlantUML Installation & Setup Troubleshooting Guide

Common errors related to Java, Graphviz, plantuml.jar configuration, and environment setup.

## Error #1: "Cannot find java!"

**Error Message:**
```
Cannot find java! If you've installed java, please add java bin path to PATH environment variable.
Or, configure java executable path with "plantuml.java".
```

**Cause:**
- Java is not installed on the system
- Java is installed but not in the system PATH
- JAVA_HOME environment variable not configured

**Solution:**

1. Verify Java installation:
```bash
java -version
```

2. If not installed, install Java:
```bash
# macOS (using Homebrew)
brew install openjdk@11

# Ubuntu/Debian
sudo apt-get install default-jdk

# Windows
# Download from https://www.oracle.com/java/technologies/downloads/
```

3. Add Java to PATH (if installed but not found):
```bash
# macOS/Linux - Add to ~/.bashrc or ~/.zshrc
export JAVA_HOME=/Library/Java/JavaVirtualMachines/jdk-11.jdk/Contents/Home
export PATH=$JAVA_HOME/bin:$PATH

# Windows - System Environment Variables
# Add C:\Program Files\Java\jdk-11\bin to PATH
```

4. For IDE/plugin users, configure Java path in settings:
```json
// VSCode settings.json
{
  "plantuml.java": "/usr/bin/java"
}
```

---

## Error #2: "No Dot executable found. Cannot find Graphviz."

**Error Message:**
```
No Dot executable found
Cannot find Graphviz.
You should try

@startuml
testdot
@enduml

or

java -jar plantuml.jar -testdot
```

**Cause:**
- Graphviz is not installed
- Graphviz is installed but `dot` executable is not in PATH
- Note: Sequence diagrams work without Graphviz, but class, state, and component diagrams require it

**Solution:**

1. Install Graphviz:
```bash
# macOS
brew install graphviz

# Ubuntu/Debian
sudo apt-get install graphviz

# Windows (using Chocolatey)
choco install graphviz

# Or download from https://graphviz.org/download/
```

2. Verify installation:
```bash
dot -V
# Should output: dot - graphviz version X.X.X
```

3. If installed but not found, add to PATH:
```bash
# macOS (Homebrew install)
export PATH="/usr/local/bin:$PATH"

# Windows
# Add C:\Program Files\Graphviz\bin to PATH
```

4. Test with PlantUML:
```bash
java -jar plantuml.jar -testdot
```

**Before (Error):**
```
$ java -jar plantuml.jar diagram.puml
Error: No Dot executable found
```

**After (Working):**
```
$ dot -V
dot - graphviz version 9.0.0
$ java -jar plantuml.jar diagram.puml
# Successfully generates diagram.png
```

---

## Error #3: "For some reason, dot/GraphViz has crashed"

**Error Message:**
```
For some reason, dot/GraphViz has crashed.
You should send this diagram and this error to plantuml@gmail.com
```

**Cause:**
- Corrupted Graphviz installation
- Version incompatibility between PlantUML and Graphviz
- Missing Graphviz dependencies
- Malformed diagram causing Graphviz crash

**Solution:**

1. Reinstall Graphviz:
```bash
# macOS
brew uninstall graphviz
brew install graphviz

# Ubuntu
sudo apt-get remove graphviz
sudo apt-get install graphviz
```

2. Update to latest PlantUML version:
```bash
# Download latest from https://plantuml.com/download
wget https://github.com/plantuml/plantuml/releases/download/v1.2025.0/plantuml.jar
```

3. Test with simple diagram:
```plantuml
@startuml
class A
class B
A --> B
@enduml
```

4. Try alternative renderer (if Graphviz continues to fail):
```plantuml
@startuml
!pragma layout smetana
class A
class B
A --> B
@enduml
```

---

## Error #4: "Unable to access jarfile plantuml.jar"

**Error Message:**
```
Error: Unable to access jarfile plantuml.jar
```

**Cause:**
- plantuml.jar file does not exist at the specified path
- Incorrect file path in command
- File permissions issue
- Working directory issue

**Solution:**

1. Verify plantuml.jar exists:
```bash
ls -l plantuml.jar
# or
ls -l ~/plantuml.jar
```

2. Download if missing:
```bash
wget https://github.com/plantuml/plantuml/releases/download/v1.2025.0/plantuml.jar
```

3. Use absolute path:
```bash
# Instead of:
java -jar plantuml.jar diagram.puml

# Use:
java -jar /Users/username/plantuml.jar diagram.puml
```

4. Check file permissions:
```bash
chmod 644 plantuml.jar
```

**Before (Error):**
```bash
$ java -jar plantuml.jar diagram.puml
Error: Unable to access jarfile plantuml.jar
```

**After (Working):**
```bash
$ java -jar ~/plantuml.jar diagram.puml
# Successfully processes diagram
```

---

## Error #5: File Permission Denied

**Error Message:**
```
Permission denied: plantuml.jar
Cannot execute jar file
```

**Cause:**
- JAR file or wrapper script lacks execute permissions
- File ownership issues
- SELinux or AppArmor restrictions (Linux)

**Solution:**

1. Fix file permissions:
```bash
chmod 644 plantuml.jar
```

2. For wrapper scripts:
```bash
chmod 755 plantuml.sh
```

3. Check file ownership:
```bash
ls -l plantuml.jar
# If owned by different user:
sudo chown $USER plantuml.jar
```

4. For Docker/container environments:
```dockerfile
COPY plantuml.jar /app/plantuml.jar
RUN chmod 644 /app/plantuml.jar
```

---

## Error #6: HeadlessException (Unix/Linux)

**Error Message:**
```
java.awt.HeadlessException
    at java.awt.GraphicsEnvironment.checkHeadless
    at java.awt.Window.<init>
```

**Cause:**
- Running PlantUML on a server without X11 display
- Missing X11 libraries
- Not using headless mode flag

**Solution:**

1. Use headless mode flag:
```bash
java -Djava.awt.headless=true -jar plantuml.jar diagram.puml
```

2. For Docker/containers:
```dockerfile
ENV JAVA_OPTS="-Djava.awt.headless=true"
CMD java $JAVA_OPTS -jar plantuml.jar -tpng /diagrams/*.puml
```

3. Install X11 libraries (if needed):
```bash
# Ubuntu/Debian
sudo apt-get install libx11-dev libxrender-dev libxext-dev

# CentOS/RHEL
sudo yum install libX11-devel libXrender-devel libXext-devel
```

**Before (Error):**
```bash
$ java -jar plantuml.jar diagram.puml
Exception in thread "main" java.awt.HeadlessException
```

**After (Working):**
```bash
$ java -Djava.awt.headless=true -jar plantuml.jar diagram.puml
# Successfully generates diagram
```

---

## Error #7: JAVA_HOME Not Set

**Error Message:**
```
JAVA_HOME is not set and no 'java' command could be found
```

**Cause:**
- JAVA_HOME environment variable not configured
- Scripts relying on JAVA_HOME cannot find Java

**Solution:**

1. Find Java installation:
```bash
# macOS
/usr/libexec/java_home

# Linux
which java
readlink -f $(which java)
```

2. Set JAVA_HOME:
```bash
# macOS/Linux - Add to ~/.bashrc or ~/.zshrc
export JAVA_HOME=/Library/Java/JavaVirtualMachines/jdk-11.jdk/Contents/Home
export PATH=$JAVA_HOME/bin:$PATH

# Or for Linux:
export JAVA_HOME=/usr/lib/jvm/java-11-openjdk-amd64
export PATH=$JAVA_HOME/bin:$PATH
```

3. Reload shell configuration:
```bash
source ~/.bashrc
# or
source ~/.zshrc
```

4. Verify:
```bash
echo $JAVA_HOME
# Should output: /path/to/java/home
```

---

## Error #8: Wrong Java Version

**Error Message:**
```
UnsupportedClassVersionError: net/sourceforge/plantuml/Run has been compiled by a more recent version of the Java Runtime
```

**Cause:**
- PlantUML JAR compiled with newer Java version than installed
- Running PlantUML requires Java 8 or higher

**Solution:**

1. Check current Java version:
```bash
java -version
```

2. Upgrade Java if needed:
```bash
# macOS
brew install openjdk@11

# Ubuntu/Debian
sudo apt-get install openjdk-11-jdk

# Update alternatives (Linux)
sudo update-alternatives --config java
```

3. Verify PlantUML works:
```bash
java -jar plantuml.jar -version
```

**Before (Error):**
```bash
$ java -version
java version "1.7.0_80"
$ java -jar plantuml.jar diagram.puml
UnsupportedClassVersionError
```

**After (Working):**
```bash
$ java -version
java version "11.0.12"
$ java -jar plantuml.jar diagram.puml
# Successfully processes diagram
```

---

## Error #9: Java Symlink Issues (macOS with sdkman)

**Error Message:**
```
Cannot find Java executable
Java found but unable to execute
```

**Cause:**
- Java executable hidden behind symlinks (common with sdkman, jenv)
- IDE/plugin cannot resolve symlinked Java path

**Solution:**

1. Find real Java path:
```bash
which java
# Output: /Users/user/.sdkman/candidates/java/current/bin/java

readlink -f $(which java)
# Output: /Users/user/.sdkman/candidates/java/11.0.12-open/bin/java
```

2. Use real path in configuration:
```json
// VSCode settings.json
{
  "plantuml.java": "/Users/user/.sdkman/candidates/java/11.0.12-open/bin/java"
}
```

3. Alternative: Create direct symlink:
```bash
sudo ln -s /Users/user/.sdkman/candidates/java/current/bin/java /usr/local/bin/java
```

---

## Error #10: Missing Dependencies in Docker/CI

**Error Message:**
```
Diagram not rendering (silent failure)
Build succeeds but no image generated
```

**Cause:**
- Essential packages not installed in container
- Missing fonts for text rendering
- Missing Graphviz in Docker image

**Solution:**

1. Complete Dockerfile setup:
```dockerfile
FROM openjdk:11-jre-slim

# Install Graphviz and fonts
RUN apt-get update && \
    apt-get install -y \
    graphviz \
    fonts-dejavu \
    fonts-liberation \
    fontconfig && \
    rm -rf /var/lib/apt/lists/*

# Copy PlantUML
COPY plantuml.jar /app/plantuml.jar

# Set headless mode
ENV JAVA_OPTS="-Djava.awt.headless=true"

WORKDIR /diagrams
CMD java $JAVA_OPTS -jar /app/plantuml.jar -tpng *.puml
```

2. For CI/CD (GitHub Actions):
```yaml
- name: Setup PlantUML
  run: |
    sudo apt-get update
    sudo apt-get install -y graphviz fonts-dejavu
    wget https://github.com/plantuml/plantuml/releases/download/v1.2025.0/plantuml.jar

- name: Generate diagrams
  run: |
    java -Djava.awt.headless=true -jar plantuml.jar -tpng docs/**/*.puml
```

---

## Error #11: PlantUML Server Connection Issues

**Error Message:**
```
Cannot connect to PlantUML server
Connection timeout
```

**Cause:**
- Network/firewall blocking PlantUML server (plantuml.com)
- Corporate proxy issues
- Server temporarily down

**Solution:**

1. Use local PlantUML instead:
```bash
# Instead of online server, use local jar
java -jar plantuml.jar diagram.puml
```

2. Configure proxy (if needed):
```bash
java -Dhttp.proxyHost=proxy.company.com -Dhttp.proxyPort=8080 -jar plantuml.jar diagram.puml
```

3. For VSCode plugin, use local rendering:
```json
{
  "plantuml.render": "Local",
  "plantuml.jar": "/path/to/plantuml.jar"
}
```

4. Self-host PlantUML server:
```bash
docker run -d -p 8080:8080 plantuml/plantuml-server:jetty
```

---

## Error #12: IDE Plugin Not Finding PlantUML

**Error Message:**
```
PlantUML not configured
Cannot preview diagram
```

**Cause:**
- Plugin cannot auto-detect plantuml.jar
- Java path not configured
- Plugin settings incomplete

**Solution:**

1. **VSCode** - Configure settings:
```json
{
  "plantuml.jar": "/Users/username/plantuml.jar",
  "plantuml.java": "/usr/bin/java",
  "plantuml.render": "Local"
}
```

2. **IntelliJ IDEA** - Configure in Settings:
```
Settings → Languages & Frameworks → PlantUML
- Graphviz dot executable: /usr/local/bin/dot
- PlantUML jar: /Users/username/plantuml.jar
```

3. **Emacs** (plantuml-mode):
```elisp
(setq plantuml-jar-path "/Users/username/plantuml.jar")
(setq plantuml-default-exec-mode 'jar)
```

4. Test configuration:
```bash
# Verify paths are correct
ls -l /Users/username/plantuml.jar
which java
which dot
```

---

## Error #13: Character Encoding Issues

**Error Message:**
```
Illegal character in diagram
Encoding error
```

**Cause:**
- .puml file not saved in UTF-8 encoding
- Special characters not properly encoded

**Solution:**

1. Save files as UTF-8:
```bash
# Verify file encoding
file -I diagram.puml

# Convert if needed
iconv -f ISO-8859-1 -t UTF-8 diagram.puml > diagram_utf8.puml
```

2. Specify encoding explicitly:
```bash
java -Dfile.encoding=UTF-8 -jar plantuml.jar diagram.puml
```

3. For IDE, set file encoding to UTF-8:
```
VSCode: Files → Preferences → Settings → Files: Encoding → UTF-8
```

---

## Error #14: PlantUML Version Mismatch

**Error Message:**
```
Requires PlantUML version >= X.Y.Z
Feature not supported in this version
```

**Cause:**
- Using outdated PlantUML version
- Diagram uses new syntax not available in old version
- Plugin/library requires newer PlantUML

**Solution:**

1. Check current version:
```bash
java -jar plantuml.jar -version
```

2. Download latest version:
```bash
wget https://github.com/plantuml/plantuml/releases/download/v1.2025.0/plantuml.jar
```

3. Update plugin reference:
```json
{
  "plantuml.jar": "/path/to/new/plantuml.jar"
}
```

4. For specific version requirements, download from releases:
```
https://github.com/plantuml/plantuml/releases
```

---

## Error #15: Environment Variables Not Recognized

**Error Message:**
```
PLANTUML_JAR not found
Environment variable not set
```

**Cause:**
- Scripts expect PLANTUML_JAR or similar environment variable
- Variable not exported to shell environment

**Solution:**

1. Set environment variable:
```bash
# Add to ~/.bashrc or ~/.zshrc
export PLANTUML_JAR=/Users/username/plantuml.jar
export PLANTUML_LIMIT_SIZE=8192
```

2. Reload shell:
```bash
source ~/.bashrc
```

3. Verify:
```bash
echo $PLANTUML_JAR
# Should output: /Users/username/plantuml.jar
```

4. For system-wide (Linux):
```bash
sudo nano /etc/environment
# Add:
PLANTUML_JAR="/usr/local/bin/plantuml.jar"
```

---

## Quick Verification Checklist

After setup, verify everything works:

```bash
# 1. Java installed and in PATH
java -version
# Expected: java version "11.x.x" or higher

# 2. Graphviz installed and in PATH
dot -V
# Expected: dot - graphviz version X.X.X

# 3. PlantUML jar accessible
ls -l ~/plantuml.jar
# Expected: File exists

# 4. PlantUML runs
java -jar ~/plantuml.jar -version
# Expected: PlantUML version X.XXXX

# 5. Graphviz integration works
java -jar ~/plantuml.jar -testdot
# Expected: "Installation seems OK. File generation OK"

# 6. Test simple diagram
echo '@startuml
actor User
User -> System : test
@enduml' > test.puml

java -jar ~/plantuml.jar test.puml
ls -l test.png
# Expected: test.png file created
```

If all checks pass, your PlantUML environment is correctly configured.

## Additional Resources

- [Official PlantUML Installation Guide](https://plantuml.com/starting)
- [PlantUML FAQ - Installation](https://plantuml.com/faq-install)
- [Graphviz Downloads](https://graphviz.org/download/)
- [Java Downloads](https://www.oracle.com/java/technologies/downloads/)
