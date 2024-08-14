#!/bin/bash
   echo "--- System Information ---"
   uname -a
   echo
   echo "--- Current User ---"
   id
   echo
   echo "--- Available Capabilities ---"
   capsh --print
   echo
   echo "--- Directory Permissions ---"
   ls -la /home/user/writable /etc
   echo
   echo "--- Mount Points ---"
   mount
   echo
   echo "--- SELinux Status ---"
   if command -v getenforce &> /dev/null; then
       getenforce
   else

       echo "SELinux tools not installed"
   fi
   echo
   echo "--- AppArmor Status ---"
   if command -v aa-status &> /dev/null; then
       aa-status
   else
       echo "AppArmor tools not installed"
   fi
   echo
   echo "--- Seccomp Status ---"
   grep Seccomp /proc/1/status
   echo
   echo "--- Landlock Kernel Support ---"
   if grep -q landlock /proc/filesystems; then
       echo "Landlock is supported by the kernel"
   else
       echo "Landlock is not supported by the kernel"
   fi
