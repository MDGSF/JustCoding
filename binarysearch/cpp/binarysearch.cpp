#include <stdio.h>

int iBinarySearch(int aiNum[], int iNumSize, int iVal) {
	if (NULL == aiNum || iNumSize <= 0) {
		return -1;
	}

	int iStart = 0;
	int iEnd = iNumSize - 1;

	while (iStart <= iEnd) {
		int iMid = iStart + ((iEnd - iStart) >> 1);
		if (aiNum[iMid] < iVal) {
			iStart = iMid + 1;
		} else if (aiNum[iMid] > iVal) {
			iEnd = iMid - 1;
		} else {
			return iMid;
		}
	}

	return -1;
}

void vTest(const char* pcCaseName, int aiNum[], int iNumSize, int iVal, int iExpectVal) {
	int iRet = iBinarySearch(aiNum, iNumSize, iVal);
	if (iRet == iExpectVal) {
		printf("%s: pass\n", pcCaseName);
	} else {
		printf("%s: failed\n", pcCaseName);
	}
}

void vTest1() {
	int ai[] = {1, 2, 3, 4, 5, 6, 7};
	int iSize = sizeof ai / sizeof ai[0];
	int iVal = 4;
	vTest("vTest1", ai, iSize, iVal, 3);
}

void vTest2() {
	int ai[] = {1, 2, 3, 4, 5, 6, 7};
	int iSize = sizeof ai / sizeof ai[0];
	int iVal = 8;
	vTest("vTest2", ai, iSize, iVal, -1);
}

void vTest3() {
	int ai[] = {1, 2, 3, 4, 5, 6, 7};
	int iSize = sizeof ai / sizeof ai[0];
	int iVal = -1;
	vTest("vTest3", ai, iSize, iVal, -1);
}

void vTest4() {
	int ai[] = {1, 2, 3, 4, 5, 6, 7};
	int iSize = sizeof ai / sizeof ai[0];
	int iVal = 1;
	vTest("vTest4", ai, iSize, iVal, 0);
}

void vTest5() {
	int ai[] = {1, 2, 3, 4, 5, 6, 7};
	int iSize = sizeof ai / sizeof ai[0];
	int iVal = 7;
	vTest("vTest5", ai, iSize, iVal, 6);
}

void vTest6() {
	int ai[] = {1, 2, 3, 4, 5, 6, 7};
	int iSize = sizeof ai / sizeof ai[0];
	int iVal = 6;
	vTest("vTest6", ai, iSize, iVal, 5);
}

void vTest7() {
	int ai[] = {1};
	int iSize = sizeof ai / sizeof ai[0];
	int iVal = 1;
	vTest("vTest7", ai, iSize, iVal, 0);
}

void vTest8() {
	int iVal = 1;
	vTest("vTest8", NULL, 0, iVal, -1);
}

int main() {
	vTest1();
	vTest2();
	vTest3();
	vTest4();
	vTest5();
	vTest6();
	vTest7();
	vTest8();
	return 0;
}
